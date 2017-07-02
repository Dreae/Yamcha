use mio::net::UdpSocket;
use mio::{Poll, Ready, PollOpt, Token, Events};
use std::net::SocketAddr;

const SERVER: Token = Token(0);

pub fn init() {
  println!("Starting Yamcha Ingress");
  let addr: SocketAddr = match "0.0.0.0:2000".parse() {
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
            let mut bytes = [0u8; 1024];
            match server.recv_from(&mut bytes) {
              Ok((num_read, _)) => {
                println!("Read {} bytes", num_read);
                println!("Log message {}", String::from_utf8_lossy(&bytes));
              },
              Err(e) => {
                println!("{}", e);
              }
            };
          },
          _ => unreachable!(),
      }
    }
  }
}