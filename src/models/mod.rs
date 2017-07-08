#[derive(Queryable)]
pub struct Server {
  pub id: i32,
  pub name: String,
  pub password: String,
  pub ip: String,
  pub port: i32,
}