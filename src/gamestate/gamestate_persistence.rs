use std::thread::{self, JoinHandle};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::Duration;
use std::borrow::Borrow;

use super::{GameState, ConnectedPlayer};
use persistence;

type OldStateStore = Arc<RwLock<HashMap<i32, ConnectedPlayer>>>;

pub struct GamestatePersistence {
  server_id: u32,
  save_thread: Option<JoinHandle<()>>,
  state: Arc<RwLock<GameState>>,
  old_state: OldStateStore,
}

impl GamestatePersistence {
  pub fn new(server_id: u32, state: Arc<RwLock<GameState>>) -> GamestatePersistence {
    GamestatePersistence {
      server_id: server_id,
      save_thread: None,
      state: state,
      old_state: Arc::new(RwLock::new(HashMap::new()))
    }
  }

  pub fn init(&mut self) {
    let state = self.state.clone();
    let server_id = self.server_id;
    let old_state = self.old_state.clone();
    info!("Starting persistence thread for server {}", server_id);
    self.save_thread = Some(thread::spawn(move || {
      loop {
        thread::sleep(Duration::from_secs(10));
        let lock = get_read_lock!(state);
        for (uid, player) in lock.players.iter() {
          Self::handle_player_update(old_state.borrow(), server_id, *uid, player, false);
        }
      }
    }));
  }

  pub fn player_disconnected(&self, uid: i32, player: &ConnectedPlayer) {
    Self::handle_player_update(self.old_state.borrow(), self.server_id, uid, player, true);
  }

  fn handle_player_update(old_state: &RwLock<HashMap<i32, ConnectedPlayer>>, server_id: u32, uid: i32, current_player: &ConnectedPlayer, disconnected: bool) {
    if current_player.bot {
      return;
    }

    match persistence::get_player(server_id as i32, &current_player.steamid) {
      Some(mut db_player) => {
        {
          db_player.rating = current_player.rating;
          db_player.last_name = current_player.name.clone();
          
          match get_write_lock!(old_state).remove(&uid) {
            Some(old_player) => {
              db_player.kills += (current_player.kills - old_player.kills) as i32;
              db_player.deaths += (current_player.deaths - old_player.deaths) as i32;
              db_player.headshots += (current_player.headshots - old_player.headshots) as i32;
              db_player.shots_fired = current_player.shots_fired - old_player.shots_fired;
              db_player.shots_hit = current_player.shots_hit - old_player.shots_hit;
            },
            None => {
              db_player.kills = current_player.kills as i32;
              db_player.deaths = current_player.deaths as i32;
              db_player.headshots = current_player.headshots as i32;
              db_player.shots_fired = current_player.shots_fired;
              db_player.shots_hit = current_player.shots_hit;
            }
          }
        }
        persistence::update_player(&db_player);

        if !disconnected {
          get_write_lock!(old_state).insert(uid, current_player.clone());
        }
      },
      None => {
        persistence::new_player(
          server_id as i32, 
          &current_player.steamid, 
          &current_player.name, 
          current_player.rating, 
          current_player.kills, 
          current_player.deaths, 
          current_player.headshots,
          current_player.shots_fired,
          current_player.shots_hit
        );
      }
    }
  }
}