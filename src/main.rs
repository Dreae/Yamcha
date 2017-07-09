#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate yamcha_macros;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;

extern crate mio;
extern crate r2d2;
extern crate regex;
extern crate dotenv;
extern crate rocket;
extern crate env_logger;
extern crate r2d2_diesel;
extern crate yamcha_rcon;
extern crate rocket_contrib;

use std::env;
use std::sync::RwLock;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;

mod ingress;
mod gamestate;
mod api;
mod middleware;
mod persistence;

pub mod schema;
pub mod models;

use gamestate::servers::{Servers, Server};

use std::thread;

lazy_static! {
    pub static ref SERVERS: RwLock<Servers> = RwLock::new(Servers::new());
    pub static ref POOL: r2d2::Pool<ConnectionManager<MysqlConnection>> = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL to be set");
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        r2d2::Pool::new(config, manager).expect("Failed to create connection pool")
    };
}

embed_migrations!("./migrations");

fn main() {
    dotenv().ok();
    env_logger::init().unwrap();
    
    yamcha_rcon::init();

    // {
    //     let conn = middleware::DBConnection::new().expect("Error getting connection from pool");
    //     embedded_migrations::run(&*conn).expect("Error running migrations");
    // }

    load_server_list();

    
    thread::spawn(move || {
        ingress::init();
    });
    
    info!("Starting web client");
    rocket::ignite().mount("/api", routes![
        api::public::get_servers,
        api::public::get_server_details,
        api::public::get_server_active_player,
    ]).launch();
}

fn load_server_list() {
    use schema::servers::dsl::*;

    let conn = middleware::DBConnection::new().expect("Error getting connection from pool");
    let results = servers.load::<models::Server>(&*conn).expect("Error loading severs");
    for server in results {
        debug!("Server {:?}", server);
        info!("Loaded server {}", server.id);
        let server_result = Server::new(server.id as u32, server.name, server.password, server.ip, server.port as u32);
        match server_result {
            Ok(mut s) => {
                s.init();
                get_write_lock!(SERVERS).register(s);
            },
            Err(e) => {
                error!("Error registering server {:?}", e);
            }
        }
    }
}