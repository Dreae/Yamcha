extern crate mio;
extern crate futures;
extern crate byteorder;

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate yamcha_macros;

use mio::{Poll, Events, Ready, PollOpt};
use std::sync::{RwLock, Arc};
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
    static ref CONNECTIONS: RwLock<HashMap<Token, Arc<RwLock<Connection>>>> = RwLock::new(HashMap::new());
}

pub fn init() {
    thread::spawn(move || {
        let mut events = Events::with_capacity(1024);

        loop {
            POLL.poll(&mut events, None).unwrap();
            
            for event in events.iter() {
                let tok = event.token();
                let connections = get_read_lock!(CONNECTIONS);
                connections.get(&tok).map(|conn| {
                    let readiness = event.readiness();

                    if readiness.is_readable() {
                        read_conn_stream(conn);
                    }

                    if readiness.is_writable() {
                        let mut conn = get_write_lock!(conn);
                        conn.writable();
                    }
                });
            }
        }
    });
}

#[inline(always)]
fn read_conn_stream(conn: &Arc<RwLock<Connection>>) {
    let mut conn = get_write_lock!(conn);
    let mut bytes: [u8; 1024] = unsafe {
        mem::uninitialized()
    };

    let mut buf = Vec::new();
    loop {
        match conn.stream.read(&mut bytes) {
            Ok(num_read) => buf.extend(bytes[0..num_read].iter()),
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

fn register(conn: Arc<RwLock<Connection>>) {
    let token = {
        let conn = get_read_lock!(conn);
        POLL.register(&conn.stream, conn.token, Ready::readable() | Ready::writable(), PollOpt::edge()).unwrap();
        
        conn.token
    };

    let mut connections = get_write_lock!(CONNECTIONS);
    connections.insert(token, conn);
}