extern crate mio;
extern crate futures;
extern crate byteorder;

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate yamcha_macros;

use mio::{Poll, Events, Ready, PollOpt};
use std::sync::{RwLock, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use std::io::{ErrorKind, Read};
use std::thread;
use mio::Token;
use std::mem;

pub mod connection;
mod packet;

use connection::Connection;

lazy_static! {
    static ref POLL: Poll = Poll::new().unwrap();
    static ref CONNECTIONS: RwLock<HashMap<Token, Arc<Connection>>> = RwLock::new(HashMap::new());
    static ref SHUTDOWN: AtomicBool = AtomicBool::new(false);
}

pub fn init() {
    thread::spawn(move || {
        let mut events = Events::with_capacity(1024);

        while !SHUTDOWN.load(Ordering::SeqCst) {
            POLL.poll(&mut events, None).unwrap();
            
            for event in events.iter() {
                let tok = event.token();
                let connections = get_read_lock!(CONNECTIONS);
                connections.get(&tok).map(|conn| {
                    let readiness = event.readiness();

                    if readiness.is_readable() {
                        read_conn_stream(conn.clone());
                    }

                    if readiness.is_writable() {
                        conn.writable();
                    }
                });
            }
        }
    });
}

pub fn shutdown() {
    SHUTDOWN.swap(true, Ordering::Relaxed);
}

#[inline(always)]
fn read_conn_stream(conn: Arc<Connection>) {
    let mut bytes: [u8; 1024] = unsafe {
        mem::uninitialized()
    };

    let mut buf = Vec::new();
    loop {
        match lock!(conn.stream).read(&mut bytes) {
            Ok(num_read) => buf.push(bytes[0..num_read].to_vec()),
            Err(e) => {
                if e.kind() != ErrorKind::WouldBlock {
                    conn.handle_stream_error(e);
                }

                break;
            }
        }
    }

    conn.read_notify(buf);
}

fn register(mut conn: Connection) -> Arc<Connection> {
    let token = {
        POLL.register(conn.stream.get_mut().unwrap(), conn.token, Ready::readable() | Ready::writable(), PollOpt::edge()).unwrap();
        
        conn.token
    };

    let conn = Arc::new(conn);

    let mut connections = get_write_lock!(CONNECTIONS);
    connections.insert(token, conn.clone());

    conn
}