extern crate mio;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

extern crate regex;
extern crate env_logger;

mod ingress;
mod gamestate;

fn main() {
    env_logger::init().unwrap();

    ingress::init();
}
