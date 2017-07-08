use rocket_contrib::JSON;
use std::vec::Vec;

use super::super::SERVERS;
use super::super::gamestate;

#[derive(Serialize)]
pub struct ServerSummary {
  server_id: u32,
  name: String,
  players_active: usize,
  players_max: usize,
  top_player: String,
}

#[get("/servers")]
pub fn get_servers() -> JSON<Vec<ServerSummary>> {
  let mut server_summaries = Vec::new();
  for (server_id, server) in get_read_lock!(SERVERS).servers.iter() {
    server_summaries.push(ServerSummary{
      server_id: *server_id,
      name: server.name.clone(),
      players_active: get_read_lock!(server.gamestate).players.keys().count(),
      players_max: server.max_players,
      top_player: "PlaceHolder Player".to_owned(),
    });
  }

  JSON(server_summaries)
}

#[get("/servers/<server_id>")]
pub fn get_server_details(server_id: u32) -> Option<JSON<Vec<gamestate::ConnectedPlayer>>> {
  get_read_lock!(SERVERS).get_server_state(server_id).map(|s| {
    let mut connected_players = Vec::new();
    for player in get_read_lock!(s.gamestate).players.values() {
      connected_players.push(player.clone());
    }

    JSON(connected_players)
  })
}

#[get("/servers/<server_id>/players/<uid>")]
pub fn get_server_active_player(server_id: u32, uid: i32) -> Option<JSON<gamestate::ConnectedPlayer>> {
    get_read_lock!(SERVERS).get_server_state(server_id).and_then(|server| get_read_lock!(server.gamestate).get_player(uid)).and_then(|p| Some(JSON(p)))
}