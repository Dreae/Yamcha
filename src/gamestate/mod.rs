use std::collections::HashMap;

use super::ingress;

pub mod servers;

#[derive(Serialize, Clone)]
pub struct ConnectedPlayer {
  rating: i32,
  kills: u32,
  deaths: u32,
  streak: u32,
  headshots: u32,
  assists: u32,
  accuracy: f32,
  steamid: String,
}

impl ConnectedPlayer {
  pub fn new(rating: i32, steamid: String) -> ConnectedPlayer {
    ConnectedPlayer {
      rating: rating,
      kills: 0,
      deaths: 0,
      streak: 0,
      headshots: 0,
      assists: 0,
      accuracy: 0.0,
      steamid: steamid,
    }
  }
}

pub struct GameState {
  pub players: HashMap<i32, ConnectedPlayer>,
}

impl GameState {
  pub fn new() -> GameState {
    GameState {
      players: HashMap::new(),
    }
  }

  pub fn process_log_msg(&mut self, msg: &ingress::logparse::LogMessage) {
    match msg.msg_type {
      ingress::logparse::LogMessageType::PlayerKilled => {
        self.player_killed(msg, false);
      },
      ingress::logparse::LogMessageType::HeadshotKill => {
        self.player_killed(msg, true);
      },
      ingress::logparse::LogMessageType::KillAssist => {
        self.kill_assist(msg);
      },
      ingress::logparse::LogMessageType::Connected => {
        self.player_connected(msg);
      },
      ingress::logparse::LogMessageType::Disconnected => {
        self.player_disconnected(msg);
      }
    }
  }

  pub fn player_killed(&mut self, msg: &ingress::logparse::LogMessage, headshot: bool) {    
    let (new_killer_rating, new_victim_rating) = {
      let killer = self.players.get(&msg.target_uid);
      let victim = self.players.get(&msg.victim_uid.unwrap());

      if killer.is_none() {
        warn!("Killer unconnected, ignoring log message");
        return;
      }

      if victim.is_none() {
        warn!("Victim unconnected, ignoring log message");
        return;
      }

      let killer = killer.unwrap();
      let victim = victim.unwrap();

      calculate_elo(killer.rating, victim.rating)
    };

    debug!("New ratings for ({},{}) are ({},{})", msg.target, msg.victim.unwrap(), new_killer_rating, new_victim_rating);

    {
      let mut killer = self.players.get_mut(&msg.target_uid).unwrap();

      killer.rating = new_killer_rating + if headshot { 1 } else { 0 };
      killer.streak += 1;
      killer.kills += 1;
      if headshot {
        killer.headshots += 1;
      }
    }

    {
      let mut victim = self.players.get_mut(&msg.victim_uid.unwrap()).unwrap();

      victim.rating = new_victim_rating;
      victim.streak = 0;
      victim.deaths += 1;
    }

  }

  pub fn kill_assist(&mut self, msg: &ingress::logparse::LogMessage) {
    let killer = self.players.get_mut(&msg.target_uid);

    if killer.is_none() {
      warn!("Killer unconnected, ignoring log message");
      return;
    }
    let mut killer = killer.unwrap();
    killer.rating += 1;
    killer.assists += 1;
  }

  pub fn player_connected(&mut self, msg: &ingress::logparse::LogMessage) {
    debug!("New player {} connected to uid {}", msg.target, msg.target_uid);
    
    self.players.insert(msg.target_uid, ConnectedPlayer {
      rating: 1000,
      kills: 0,
      deaths: 0,
      streak: 0,
      headshots: 0,
      assists: 0,
      accuracy: 0.0,
      steamid: msg.target.to_owned(),
    });
  }

  pub fn player_disconnected(&mut self, msg: &ingress::logparse::LogMessage) {
    debug!("Player {} disconnected from uid {}", msg.target, msg.target_uid);
    self.players.remove(&msg.target_uid);
  }

  pub fn get_player(&self, uid: i32) -> Option<ConnectedPlayer> {
    self.players.get(&uid).and_then(|p| Some(p.clone()))
  }
}

#[inline(always)]
pub fn calculate_elo(killer: i32, victim: i32) -> (i32, i32) {
  const K: f64 = 16f64;
  let expected_killer = 1f64 / (1f64 + 10f64.powf(f64::from(victim - killer) / 400f64));
  let expected_victim = 1f64 - expected_killer;

  (killer + (K * (1f64 - expected_killer)).round() as i32, victim + (K * (0f64 - expected_victim)).round() as i32)
}