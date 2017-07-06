#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

extern crate mio;
extern crate regex;
extern crate rocket;
extern crate env_logger;
extern crate rocket_contrib;

use std::sync::RwLock;

#[macro_use] mod macros;
mod ingress;
mod gamestate;
mod api;

use gamestate::servers::Servers;

use std::thread;

lazy_static! {
    pub static ref SERVERS: RwLock<Servers> = RwLock::new(Servers::new());
}

fn main() {
    get_write_lock!(SERVERS).register(1);

    env_logger::init().unwrap();
    
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
