use models::Player;
use middleware::DBConnection;
use schema::players::{self, dsl};
use diesel::prelude::*;
use diesel;

pub fn get_player(server_id: i32, steam_id: &str) -> Option<Player> {
  if let Ok(conn) = DBConnection::new() {
    let res = dsl::players.filter(dsl::server_id.eq(server_id)).filter(dsl::steam_id.eq(steam_id)).limit(1).load::<Player>(&*conn);
    match res {
      Ok(mut rows) => {
        if rows.len() == 0 {
          None
        } else {
          Some(rows.remove(0))
        }
      },
      Err(e) => {
        error!("Error fetching player: {:?}", e);
        None
      },
    }
  } else {
    error!("Timeout getting database connection");
    None
  }
}

pub fn update_player(player: &Player) {
  debug!("Updating player {:?}", player);
  if let Ok(conn) = DBConnection::new() {
    let res = diesel::update(dsl::players.filter(dsl::server_id.eq(&player.server_id)).filter(dsl::steam_id.eq(&player.steam_id)))
      .set(player)
      .execute(&*conn);

    match res {
      Err(e) => error!("Error updating player {:?}", e),
      _ => { }
    };
  } else {
    error!("Timeout getting database connection");
  }
}

pub fn new_player(server_id: i32, steam_id: &str, name: &str, rating: i32, kills: u32, deaths: u32, headshots: u32, shots_fired: i64, shots_hit: i64) {
  if let Ok(conn) = DBConnection::new() {
    let player = Player {
      rating: rating,
      last_name: name.to_owned(),
      steam_id: steam_id.to_owned(),
      server_id: server_id,
      kills: kills as i32,
      deaths: deaths as i32,
      headshots: headshots as i32,
      shots_fired: shots_fired,
      shots_hit: shots_hit,
    };

    match diesel::insert(&player).into(players::table).execute(&*conn) {
      Err(e) => error!("Error updating player {:?}", e),
      _ => { }
    };
  } else {
    error!("Timeout getting database connection");
  }
}