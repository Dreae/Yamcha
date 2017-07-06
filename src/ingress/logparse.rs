use std::mem;
use std::str;

use regex::{Regex, Captures};

#[derive(Debug, PartialEq)]
pub enum ParseError {
  ByteBufferTooShort,
  HeaderMissing,
  WrongMsgType,
  RegexFail,
  NotAString,
  InvalidServerId,
}

pub type ParseResult<'a> = Result<LogMessage<'a>, ParseError>;

#[derive(Debug)]
pub enum LogMessageType {
  HeadshotKill,
  PlayerKilled,
  KillAssist,
  Connected,
  Disconnected,
}

#[derive(Debug)]
pub struct Position {
  x: i32,
  y: i32,
  z: i32,
}

#[derive(Debug)]
pub struct LogMessage<'a> {
  pub server_id: u32,
  pub msg_type: LogMessageType,
  pub target: &'a str,
  pub target_uid: i32,
  pub victim: Option<&'a str>,
  pub victim_uid: Option<i32>,
  pub weapon: Option<&'a str>,
  pub target_pos: Option<Box<Position>>,
  pub victim_pos: Option<Box<Position>>,
}

pub fn parse<'a>(bytes: &'a [u8]) -> ParseResult<'a> {  
  if bytes.len() < 5 {
    return Err(ParseError::ByteBufferTooShort);
  }

  let first_4_bytes = unsafe {
    mem::transmute::<*const u8, *const u32>(bytes[0..4].as_ptr())
  };

  unsafe {
    if *first_4_bytes != 0xffffffff {
      debug!("Expected 0xffffffff found {:x}", *first_4_bytes);
      return Err(ParseError::HeaderMissing);
    }
  }

  if bytes[4] != 0x53 {
    debug!("Expected 0x53 found {:x}", bytes[4]);
    return Err(ParseError::WrongMsgType);
  }
  
  let msg = if let Ok(string) = str::from_utf8(&bytes[5..]) {
    string
  } else {
    return Err(ParseError::NotAString)
  };
  
  lazy_static! {
    static ref KILL_RE: Regex = Regex::new(r#"1server_id_([0-9]+)[^"]*"(?:.+<([0-9]+)><([a-zA-Z0-9\-:_]+)><([A-Z]+)>)"\s*\[([0-9\-]+)\s*([0-9\-]+)\s*([0-9\-]+)\]\s*killed\s*"(?:.+<([0-9]+)><([a-zA-Z0-9\-:_]+)><([A-Z]+)>)"\s*\[([0-9\-]+)\s*([0-9\-]+)\s*([0-9\-]+)\]\s*with\s*"([^"]+)"\s*(\(headshot\))?"#).unwrap();
    static ref ASSIST_RE: Regex = Regex::new(r#"1server_id_([0-9]+)[^"]*"(?:.+<([0-9]+)><([a-zA-Z0-9\-:_]+)><([A-Z]+)>)"\s*assisted\s*killing\s*"(?:.+<([0-9]+)><([a-zA-Z0-9\-:_]+)><([A-Z]+)>)""#).unwrap();
    static ref DISCONNECT_RE: Regex = Regex::new(r#"1server_id_([0-9]+)[^"]*"(?:.+<([0-9]+)><([a-zA-Z0-9\-:_]+)><([A-Z]+)>)"\s*disconnected"#).unwrap();
    static ref CONNECT_RE: Regex = Regex::new(r#"1server_id_([0-9]+)[^"]*"(?:.+<([0-9]+)><([a-zA-Z0-9\-:_]+)><([A-Z]*)>)"\s*entered the game"#).unwrap();
  }

  match KILL_RE.captures(msg) {
    Some(m) => parse_kill_msg(&m),
    None => Err(ParseError::RegexFail)
  }.or_else(|_| {
    match ASSIST_RE.captures(msg) {
      Some(m) => parse_assist_msg(&m),
      None => Err(ParseError::RegexFail)
    }
  }).or_else(|_| {
    match CONNECT_RE.captures(msg) {
      Some(m) => parse_connected(&m, LogMessageType::Connected),
      None => Err(ParseError::RegexFail)
    }
  }).or_else(|_| {
    match DISCONNECT_RE.captures(msg) {
      Some(m) => parse_connected(&m, LogMessageType::Disconnected),
      None => Err(ParseError::RegexFail)
    }
  })
}

macro_rules! get_server_id {
    ($m:ident) => {
      match $m.get(1).map_or("-1", |g| g.as_str()).parse::<u32>() {
        Ok(id) => id,
        Err(_) => return Err(ParseError::InvalidServerId)
      }
    }
}

fn parse_kill_msg<'a>(m: &Captures<'a>) -> ParseResult<'a> {
  let server_id = get_server_id!(m);

  let msg_type = if let Some(_) = m.get(15) {
    LogMessageType::HeadshotKill
  } else {
    LogMessageType::PlayerKilled
  };
  
  let target_uid = m.get(2).map_or("-1", |g| g.as_str()).parse::<i32>().unwrap_or(-1);
  let victim_uid = m.get(8).map_or("-1", |g| g.as_str()).parse::<i32>().unwrap_or(-1);

  if target_uid == -1 || victim_uid == -1 {
    return Err(ParseError::RegexFail);
  }

  Ok(LogMessage {
    server_id: server_id,
    msg_type: msg_type,
    target: m.get(3).map_or("", |g| g.as_str()),
    target_uid: target_uid,
    victim: Some(m.get(9).map_or("", |g| g.as_str())),
    victim_uid: Some(victim_uid),
    weapon: m.get(14).map(|g| g.as_str()),
    target_pos: Some(Box::new(Position{
      x: m.get(5).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
      y: m.get(6).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
      z: m.get(7).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
    })),
    victim_pos: Some(Box::new(Position{
      x: m.get(11).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
      y: m.get(12).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
      z: m.get(13).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
    })),
  })
}

fn parse_assist_msg<'a>(m: &Captures<'a>) -> ParseResult<'a> {
  let server_id = get_server_id!(m);

  let target_uid = m.get(2).map_or("-1", |g| g.as_str()).parse::<i32>().unwrap_or(-1);
  let victim_uid = m.get(5).map_or("-1", |g| g.as_str()).parse::<i32>().unwrap_or(-1);

  if target_uid == -1 || victim_uid == -1 {
    return Err(ParseError::RegexFail);
  }

  Ok(LogMessage {
    server_id: server_id,
    msg_type: LogMessageType::KillAssist,
    target: m.get(3).map_or("", |g| g.as_str()),
    target_uid: target_uid,
    victim: Some(m.get(6).map_or("", |g| g.as_str())),
    victim_uid: Some(victim_uid),
    weapon: None,
    target_pos: None,
    victim_pos: None,
  })
}

fn parse_connected<'a>(m: &Captures<'a>, msg_type: LogMessageType) -> ParseResult<'a> {
  let server_id = get_server_id!(m);

  let target_uid = m.get(2).map_or("-1", |g| g.as_str()).parse::<i32>().unwrap_or(-1);

  if target_uid == -1 {
    return Err(ParseError::RegexFail);
  }

  Ok(LogMessage {
    server_id: server_id,
    msg_type: msg_type,
    target: m.get(3).map_or("", |g| g.as_str()),
    target_uid: target_uid,
    victim: None,
    victim_uid: None,
    weapon: None,
    target_pos: None,
    victim_pos: None,
  })
}