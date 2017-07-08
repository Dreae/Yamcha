use rocket::request::{Request, FromRequest};
use rocket::outcome::Outcome;
use rocket::http::Status;
use rocket::request;
use std::ops::Deref;
use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::PooledConnection;
use super::POOL;
use r2d2;

pub struct DBConnection {
  conn: PooledConnection<ConnectionManager<MysqlConnection>>,
}

impl DBConnection {
  pub fn new() -> Result<DBConnection, r2d2::GetTimeout> {
    let pool = POOL.clone().get()?;
    
    Ok(DBConnection {
      conn: pool,
    })
  }
}

impl <'a, 'r> FromRequest<'a, 'r> for DBConnection {
  type Error = r2d2::GetTimeout;
  fn from_request(_: &'a Request<'r>) -> request::Outcome<DBConnection, Self::Error> {
    match DBConnection::new() {
      Ok(conn) => Outcome::Success(conn),
      Err(e) => Outcome::Failure((Status::ServiceUnavailable, e)),
    }
  }
}

impl Deref for DBConnection {
  type Target = MysqlConnection;

  fn deref(&self) -> &MysqlConnection {
    &*self.conn
  }
}