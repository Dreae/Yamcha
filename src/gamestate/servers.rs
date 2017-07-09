use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use yamcha_rcon::connection::Connection;
use yamcha_rcon;
use regex::Regex;

use super::{GameState, ConnectedPlayer, ingress};
use super::gamestate_persistence::GamestatePersistence;
use persistence;

pub struct Server {
  pub server_id: u32,
  pub name: String,
  pub gamestate: Arc<RwLock<GameState>>,
  pub max_players: usize,
  rcon_address: String,
  rcon_conn: Arc<Connection>,
  persistence_thread: Option<GamestatePersistence>,
}

impl Server {
  pub fn new(id: u32, name: String, rcon_password: String, rcon_address: String, rcon_port: u32) -> Result<Server, yamcha_rcon::connection::RconError> {
    debug!("RconAddr for {}: {}:{}", id, rcon_address, rcon_port);
    let rcon_conn = match Connection::new(&format!("{}:{}", rcon_address, rcon_port), &rcon_password) {
      Ok(conn) => conn,
      Err(e) => return Err(e),
    };
    
    Ok(Server {
      server_id: id,
      name: name,
      gamestate: Arc::new(RwLock::new(GameState::new())),
      max_players: 32,
      rcon_address: rcon_address,
      rcon_conn: rcon_conn,
      persistence_thread: None,
    })
  }

  pub fn init(&mut self) {
    debug!("Initializing server {}", self.server_id);
    match self.rcon_conn.send_cmd("status") {
      Ok(status) => self.parse_status(&status),
      Err(e) => warn!("Error fetching initial state for server {}: {:?}", self.rcon_address, e),
    };

    let mut save_thread = GamestatePersistence::new(self.server_id, self.gamestate.clone());
    save_thread.init();
    
    self.persistence_thread = Some(save_thread);
  }

  pub fn parse_status(&mut self, status: &str) {
    lazy_static! {
      static ref HOSTNAME_RE: Regex = Regex::new(r#"hostname:\s*(.+)"#).unwrap();
      static ref MAXPLAYERS_RE: Regex = Regex::new(r#"players\s*:\s*.*\(([0-9]+)"#).unwrap();
      static ref PLAYER_RE: Regex = Regex::new(r#"#\s*([0-9]+)\s*[0-9]*\s*"(.*)"\s*([a-zA-Z0-9\-:_]+)"#).unwrap();
    }
    for line in status.split("\n") {
      if HOSTNAME_RE.is_match(line) {
        self.name = HOSTNAME_RE.captures(line).unwrap().get(1).map_or("Error connecting", |g| g.as_str()).to_owned();
      } else if MAXPLAYERS_RE.is_match(line) {
        self.max_players = MAXPLAYERS_RE.captures(line).unwrap().get(1).map_or("0", |g| g.as_str()).parse::<usize>().unwrap_or(0);
      } else if PLAYER_RE.is_match(line) {
        let m = PLAYER_RE.captures(line).unwrap();
        let uid = m.get(1).map_or("0", |g| g.as_str()).parse::<i32>().unwrap_or(0);
        let steamid = match m.get(3) {
          Some(g) => g.as_str(),
          None => "BOT",
        }.to_owned();
        let name = match m.get(2) {
          Some(n) => n.as_str(),
          None => "",
        }.to_owned();

        if steamid != "BOT" {
          match persistence::get_player(self.server_id as i32, &steamid) {
            Some(player) => get_write_lock!(self.gamestate).players.insert(uid, ConnectedPlayer::new(player.rating, steamid, name)),
            None => get_write_lock!(self.gamestate).players.insert(uid, ConnectedPlayer::new(1000, steamid, name)),
          };
        } else {
          get_write_lock!(self.gamestate).players.insert(uid, ConnectedPlayer::new(1000, steamid, String::new()));
        }
      }
    }
  }

  pub fn process_log_msg(&self, msg: &ingress::logparse::LogMessage) {
    match msg.msg_type {
      ingress::logparse::LogMessageType::Connected => {
        if msg.target != "BOT" {
          match persistence::get_player(self.server_id as i32, msg.target) {
            Some(player) => get_write_lock!(self.gamestate).players.insert(msg.target_uid, ConnectedPlayer::new(player.rating, player.steam_id, msg.target_name.unwrap().to_owned())),
            None => get_write_lock!(self.gamestate).players.insert(msg.target_uid, ConnectedPlayer::new(1000, msg.target.to_owned(), msg.target_name.unwrap().to_owned())),
          };
        } else {
          get_write_lock!(self.gamestate).players.insert(msg.target_uid, ConnectedPlayer::new(1000, msg.target.to_owned(), msg.target_name.unwrap().to_owned()));
        }
      },
      ingress::logparse::LogMessageType::Disconnected => {
        match get_write_lock!(self.gamestate).players.remove(&msg.target_uid) {
          Some(connected_player) => {
            if msg.target != "BOT" {
              match self.persistence_thread {
                Some(ref persistence_thread) => persistence_thread.player_disconnected(msg.target_uid, &connected_player),
                None => warn!("Player disconnected before server initialized"),
              };
            }
          },
          None => warn!("No record of disconecting player {}", msg.target),
        };
      },
      _ => get_write_lock!(self.gamestate).process_log_msg(msg)
    };
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