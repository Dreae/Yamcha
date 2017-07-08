use super::schema::players;

#[derive(Queryable)]
pub struct Server {
  pub id: i32,
  pub name: String,
  pub password: String,
  pub ip: String,
  pub port: i32,
}

#[derive(Queryable, Clone)]
pub struct Player {
  pub steam_id: String,
  pub server_id: i32,
  pub last_name: String,
  pub rating: i32,
  pub accuracy: f32,
  pub kills: i32,
  pub deaths: i32,
  pub headshots: i32,
}

#[derive(Insertable)]
#[table_name="players"]
pub struct NewPlayer<'a> {
  pub steam_id: &'a str,
  pub server_id: i32,
}