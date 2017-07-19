use super::schema::players;

#[derive(Queryable, Debug)]
pub struct Server {
  pub id: i32,
  pub name: String,
  pub password: String,
  pub ip: String,
  pub port: i32,
}

#[derive(Queryable, Insertable, Debug, AsChangeset)]
#[table_name="players"]
pub struct Player {
  pub player_id: i32,
  pub steam_id: String,
  pub server_id: i32,
  pub last_name: String,
  pub rating: i32,
  pub shots_fired: i64,
  pub shots_hit: i64,
  pub kills: i32,
  pub deaths: i32,
  pub headshots: i32,
}

#[derive(Insertable)]
#[table_name="players"]
pub struct NewPlayer {
  pub steam_id: String,
  pub server_id: i32,
  pub last_name: String,
  pub rating: i32,
  pub shots_fired: i64,
  pub shots_hit: i64,
  pub kills: i32,
  pub deaths: i32,
  pub headshots: i32,
}