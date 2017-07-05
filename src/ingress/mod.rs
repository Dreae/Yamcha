use mio::net::UdpSocket;
use mio::{Poll, Ready, PollOpt, Token, Events};
use std::net::SocketAddr;
use std::io::ErrorKind;
use std::str;
use std::mem;

use super::gamestate;

pub mod logparse;

const SERVER: Token = Token(0);

pub fn init() {
  info!("Starting Yamcha Ingress");
  let addr: SocketAddr = match "0.0.0.0:2000".parse() {
    Ok(socket) => socket,
    Err(_) => panic!("Couldn't parse socket address"),
  };

  let server = UdpSocket::bind(&addr).unwrap();

  let poll = Poll::new().unwrap();

  poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

  let mut events = Events::with_capacity(1024);
  let mut state = gamestate::GameState::new();

  loop {
    poll.poll(&mut events, None).unwrap();

    for event in events.iter() {
      match event.token() {
          SERVER => {
            let mut bytes: [u8; 1024] = unsafe {
              mem::uninitialized()
            };

            let mut buf = Vec::new();
            loop {
              match server.recv_from(&mut bytes) {
                Ok((num_read, _)) => {
                  buf.extend(bytes[0..num_read].iter());
                },
                Err(e) => {
                  if e.kind() != ErrorKind::WouldBlock {
                    error!("Socket err: {}", e);
                  }
                  break;
                }
              };
            }
            
            for msg in buf.split(|b| *b == 0x00u8) {
              match logparse::parse(&msg) {
                Ok(msg) => {
                  debug!("{:?}", msg);
                  state.process_log_msg(&msg);
                },
                Err(reason) => {
                  debug!("Got parse fail {:?}", reason);
                  if reason != logparse::ParseError::RegexFail && reason != logparse::ParseError::ByteBufferTooShort {
                    warn!("Error parsing log message {:?}", reason);
                  }
                },
              };
            }
          },
          _ => unreachable!(),
      }
    }
  }
}