use std::collections::HashMap;
use std::sync::RwLock;

use super::GameState;

pub struct Servers {
  pub servers: HashMap<u32, RwLock<GameState>>,
}

impl Servers {
  pub fn new() -> Servers {
    Servers {
      servers: HashMap::new()
    }
  }
  
  pub fn register(&mut self, server_id: u32) {
    self.servers.insert(server_id, RwLock::new(GameState::new()));
  } 

  pub fn get_server_state(&self, server_id: u32) -> Option<&RwLock<GameState>> {
    self.servers.get(&server_id)
  }
}