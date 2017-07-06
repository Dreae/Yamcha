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

use gamestate::servers::Servers;
use rocket_contrib::JSON;

use std::thread;

lazy_static! {
    pub static ref SERVERS: RwLock<Servers> = RwLock::new(Servers::new());
}

#[get("/server/<server_id>/player/<uid>")]
fn hello(server_id: u32, uid: i32) -> Option<JSON<gamestate::ConnectedPlayer>> {
    let servers = get_read_lock!(SERVERS);

    servers.get_server_state(server_id).and_then(|server| get_read_lock!(server).get_player(uid)).and_then(|p| Some(JSON(p)))
}

fn main() {
    get_write_lock!(SERVERS).register(1);

    env_logger::init().unwrap();
    
    thread::spawn(move || {
        ingress::init();
    });
    
    info!("Starting web client");
    rocket::ignite().mount("/", routes![hello]).launch();
}
