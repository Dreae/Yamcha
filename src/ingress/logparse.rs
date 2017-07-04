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
}

pub type ParseResult<'a> = Result<LogMessage<'a>, ParseError>;

#[derive(Debug)]
pub enum LogMessageType {
  HeadshotKill,
  PlayerKilled,
  KillAssist,
}

#[derive(Debug)]
pub struct Position {
  x: i32,
  y: i32,
  z: i32,
}

#[derive(Debug)]
pub struct LogMessage<'a> {
  msg_type: LogMessageType,
  target: &'a str,
  victim: &'a str,
  weapon: Option<&'a str>,
  target_pos: Option<Box<Position>>,
  victim_pos: Option<Box<Position>>,
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
    static ref KILL_RE: Regex = Regex::new(r#""(?:.+<[0-9]+><([a-zA-Z0-9\-:_]+)><([A-Z]+)>)"\s*\[([0-9\-]+)\s*([0-9\-]+)\s*([0-9\-]+)\]\s*killed\s*"(?:.+<[0-9]+><([a-zA-Z0-9\-:_]+)><([A-Z]+)>)"\s*\[([0-9\-]+)\s*([0-9\-]+)\s*([0-9\-]+)\]\s*with\s*"([^"]+)"\s*(\(headshot\))?"#).unwrap();
    static ref ASSIST_RE: Regex = Regex::new(r#""(?:.+<[0-9]+><([a-zA-Z0-9\-:_]+)><([A-Z]+)>)"\s*assisted\s*killing\s*"(?:.+<[0-9]+><([a-zA-Z0-9\-:_]+)><([A-Z]+)>)""#).unwrap();
  }

  match KILL_RE.captures(msg) {
    Some(m) => parse_kill_msg(&m),
    None => Err(ParseError::RegexFail)
  }.or_else(|_| {
    match ASSIST_RE.captures(msg) {
      Some(m) => parse_assist_msg(&m),
      None => Err(ParseError::RegexFail)
    }
  })
}

fn parse_kill_msg<'a>(m: &Captures<'a>) -> ParseResult<'a> {
  let msg_type = if let Some(_) = m.get(12) {
    LogMessageType::HeadshotKill
  } else {
    LogMessageType::PlayerKilled
  };

  Ok(LogMessage {
    msg_type: msg_type,
    target: m.get(1).map_or("", |g| g.as_str()),
    victim: m.get(6).map_or("", |g| g.as_str()),
    weapon: m.get(11).map(|g| g.as_str()),
    target_pos: Some(Box::new(Position{
      x: m.get(3).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
      y: m.get(4).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
      z: m.get(5).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
    })),
    victim_pos: Some(Box::new(Position{
      x: m.get(8).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
      y: m.get(9).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
      z: m.get(10).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0),
    })),
  })
}

fn parse_assist_msg<'a>(m: &Captures<'a>) -> ParseResult<'a> {
  Ok(LogMessage {
    msg_type: LogMessageType::KillAssist,
    target: m.get(1).map_or("", |g| g.as_str()),
    victim: m.get(3).map_or("", |g| g.as_str()),
    weapon: None,
    target_pos: None,
    victim_pos: None,
  })
}