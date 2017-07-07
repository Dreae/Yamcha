extern crate yamcha_rcon;
extern crate futures;
extern crate env_logger;
extern crate getopts;

use yamcha_rcon::connection::Connection;
use futures::future::Future;
use futures::Async;
use std::{thread, time};
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] PASSWORD COMMAND", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    env_logger::init().unwrap();
    
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("a", "ip", "The IP address to connect to", "127.0.0.1");
    opts.optopt("p", "port", "The port to connect to", "27015");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let addr = matches.opt_str("a").unwrap_or("127.0.0.1".to_owned());
    let port: u32 = matches.opt_str("p").unwrap_or("27015".to_owned()).parse().unwrap_or(27015);

    if !matches.free.len() == 2 {
        print_usage(&program, opts);
        return;
    }

    yamcha_rcon::init();
    let con = Connection::new(&format!("{}:{}", addr, port), &matches.free[0].clone()).unwrap();

    let mut res = con.send_cmd(&matches.free[1].clone());
    
    loop {
        match res.poll() {
            Ok(state) => {
                match state {
                    Async::Ready(res) => {
                        println!("{}", res);
                        break;
                    },
                    Async::NotReady => {
                        thread::sleep(time::Duration::from_millis(100));
                     }
                }
            },
            Err(e) => {
                println!("RCON Error: {:?}", e);
                break;
            }
        }
    }
    
    yamcha_rcon::shutdown();
}
