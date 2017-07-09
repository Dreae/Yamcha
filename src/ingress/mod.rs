use mio::net::UdpSocket;
use mio::{Poll, Ready, PollOpt, Token, Events};
use std::net::SocketAddr;
use std::io::ErrorKind;
use std::{str, mem, env};

pub mod logparse;

use super::SERVERS;

const SERVER: Token = Token(0);

pub fn init() {
  info!("Starting Yamcha Ingress");
  
  let listen_ip = env::var("YAMCHA_INGRESS_ADDRESS").unwrap_or("0.0.0.0".to_owned());
  let list_port = env::var("YAMCHA_INGRESS_PORT").unwrap_or("2000".to_owned());

  let addr: SocketAddr = match format!("{}:{}", listen_ip, list_port).parse() {
    Ok(socket) => socket,
    Err(_) => panic!("Couldn't parse socket address"),
  };

  let server = UdpSocket::bind(&addr).unwrap();

  let poll = Poll::new().unwrap();

  poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

  let mut events = Events::with_capacity(1024);

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
                  get_read_lock!(SERVERS).get_server_state(msg.server_id).map(|s| get_write_lock!(s.gamestate).process_log_msg(&msg));
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