use models::Player;
use middleware::DBConnection;
use schema::players::dsl;
use diesel::prelude::*;

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