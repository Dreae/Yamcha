use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use yamcha_rcon::connection::Connection;
use yamcha_rcon;
use regex::Regex;

use super::{GameState, ConnectedPlayer};

pub struct Server {
  pub server_id: u32,
  pub name: String,
  pub gamestate: RwLock<GameState>,
  pub max_players: usize,
  rcon_password: String,
  rcon_port: u32,
  rcon_address: String,
  rcon_conn: Arc<Connection>,
}

impl Server {
  pub fn new(id: u32, name: String, rcon_password: String, rcon_address: String, rcon_port: u32) -> Result<Server, yamcha_rcon::connection::RconError> {
    let rcon_conn = match Connection::new(&format!("{}:{}", rcon_address, rcon_port), &rcon_password) {
      Ok(conn) => conn,
      Err(e) => return Err(e),
    };
    
    Ok(Server {
      server_id: id,
      name: name,
      gamestate: RwLock::new(GameState::new()),
      max_players: 32,
      rcon_password: rcon_password,
      rcon_address: rcon_address,
      rcon_port: rcon_port,
      rcon_conn: rcon_conn,
    })
  }

  pub fn rcon_init(&mut self) {
    match self.rcon_conn.send_cmd("status") {
      Ok(status) => self.parse_status(status),
      Err(e) => warn!("Error fetching initial state for server {}: {:?}", self.rcon_address, e),
    };
  }

  fn parse_status(&mut self, status: String) {
    lazy_static! {
      static ref HOSTNAME_RE: Regex = Regex::new(r#"hostname:\s*(.+)"#).unwrap();
      static ref MAXPLAYERS_RE: Regex = Regex::new(r#"players\s*:\s*.*\(([0-9]+)"#).unwrap();
      static ref PLAYER_RE: Regex = Regex::new(r#"#\s*([0-9]+)\s*[0-9]*\s*".*"\s*([a-zA-Z0-9\-:_]+)"#).unwrap();
    }
    for line in status.split("\n") {
      if HOSTNAME_RE.is_match(line) {
        self.name = HOSTNAME_RE.captures(line).unwrap().get(1).map_or("Error connecting", |g| g.as_str()).to_owned();
      } else if MAXPLAYERS_RE.is_match(line) {
        self.max_players = MAXPLAYERS_RE.captures(line).unwrap().get(1).map_or("0", |g| g.as_str()).parse::<usize>().unwrap_or(0);
      } else if PLAYER_RE.is_match(line) {
        let m = PLAYER_RE.captures(line).unwrap();
        let uid = m.get(1).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0);
        let steamid = match m.get(2) {
          Some(g) => g.as_str(),
          None => "BOT",
        }.to_owned();

        get_write_lock!(self.gamestate).players.insert(uid, ConnectedPlayer::new(1000, steamid));
      }
    }
  }
}

pub struct Servers {
  pub servers: HashMap<u32, Server>,
}

impl Servers {
  pub fn new() -> Servers {
    Servers {
      servers: HashMap::new()
    }
  }
  
  pub fn register(&mut self, server: Server) {
    self.servers.insert(server.server_id, server);
  } 

  pub fn get_server_state(&self, server_id: u32) -> Option<&Server> {
    self.servers.get(&server_id)
  }
}