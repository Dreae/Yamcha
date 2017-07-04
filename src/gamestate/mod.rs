use std::collections::HashMap;

use super::ingress;

pub enum PlayerTeam {
  CT,
  T
}

impl PlayerTeam {
  pub fn as_str(&self) -> &str {
    match self {
      &PlayerTeam::CT => "CT",
      &PlayerTeam::T => "TERRORIST"
    }
  }
}

pub struct ConnectedPlayer {
  rating: i32,
  kills: u32,
  deaths: u32,
  streak: u32,
  steamid: String,
  team: PlayerTeam,
}

pub struct GameState {
  players: HashMap<String, ConnectedPlayer>,
}

impl GameState {
  pub fn new() -> GameState {
    GameState {
      players: HashMap::new(),
    }
  }

  pub fn process_log_msg(msg: &ingress::logparse::LogMessage) {

  }
}

#[inline(always)]
pub fn calculate_elo(killer: i32, victim: i32) -> (i32, i32) {
  const K: f64 = 16f64;
  let expected_killer = 1f64 / (1f64 + 10f64.powf(f64::from(victim - killer) / 400f64));
  let expected_victim = 1f64 - expected_killer;

  (killer + (K * (1f64 - expected_killer)).round() as i32, victim + (K * (0f64 - expected_victim)).round() as i32)
}