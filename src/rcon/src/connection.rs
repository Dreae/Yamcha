use mio::Token;
use mio::net::TcpStream;
use std::io::{Error, ErrorKind, Write};
use std::collections::{VecDeque, HashMap};
use futures::{Future, Async, future};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::net::{SocketAddr, AddrParseError};

use packet::{self, PacketType};

lazy_static! {
    static ref TOKEN: AtomicUsize = AtomicUsize::new(0);
}

pub struct Connection {
    pub token: Token,
    pub stream: TcpStream,
    writable: bool,
    packet_queue: VecDeque<Vec<u8>>,
    response_map: Arc<RwLock<HashMap<i32, String>>>,
    packet_id: i32,
}

#[derive(Clone)]
pub enum RconError {
    SocketError(ErrorKind),
    ParseFail(AddrParseError),
}

type RconResult = Result<String, RconError>;

impl Connection {
    pub fn new(addr: &str, password: &str) -> Result<Arc<RwLock<Connection>>, RconError> {
        let address: SocketAddr = match addr.parse() {
            Ok(address) => address,
            Err(parse_error) => {
                return Err(RconError::ParseFail(parse_error));
            }
        };

        let stream = match TcpStream::connect(&address) {
            Ok(stream) => stream,
            Err(e) => {
                return Err(RconError::SocketError(e.kind()));
            }
        };

        let mut packet_queue = VecDeque::new();
        packet_queue.push_back(packet::build_packet(PacketType::Auth, 1, password));

        let connection = Arc::new(RwLock::new(Connection {
            token: Token(TOKEN.fetch_add(1, Ordering::SeqCst)),
            stream: stream,
            packet_id: 2,
            packet_queue: packet_queue,
            response_map: Arc::new(RwLock::new(HashMap::new())),
            writable: false,
        }));

        super::register(connection.clone());

        Ok(connection)
    }

    pub fn read_notify(&mut self, buf: Vec<u8>) {
        if buf.len() < 10 {
            warn!("Read notify was given a buffer that was too short");
        }
        for msg in buf.split(|b| *b == 0x00u8) {
            match packet::parse_packet(msg) {
                Some((packet_id, response)) => {
                    get_write_lock!(self.response_map).insert(packet_id, response);
                },
                None => warn!("Possible junk data in stream"),
            }
        }
    }

    pub fn handle_stream_error(&mut self, e: Error) {
        if e.kind() != ErrorKind::WouldBlock {
            error!("Stream error {:?}", e)
        }

        if e.kind() != ErrorKind::Interrupted {
            self.writable = false;
        }
    }

    pub fn writable(&mut self) {
        self.writable = true;
        self.pump_queue();
    }

    pub fn notify_cmd(&mut self, cmd: &str) {
        let packet_id = self.next_packet_id();
        self.packet_queue.push_back(packet::build_packet(PacketType::ExecCommand, packet_id, cmd));
    }

    pub fn send_cmd(&mut self, cmd: &str) -> Box<Future<Item=String, Error=RconError>> {
        let packet_id = self.next_packet_id();
        self.packet_queue.push_back(packet::build_packet(PacketType::ExecCommand, packet_id, cmd));
        
        let response_map = self.response_map.clone();
        Box::new(future::poll_fn(move || {
            match get_read_lock!(response_map).get(&packet_id) {
                Some(result) => Ok(Async::Ready(result.clone())),
                None => Ok(Async::NotReady)
            }
        }))
    }

    fn pump_queue(&mut self) {
        if !self.packet_queue.is_empty() {
            while self.writable {
                if let Some(packet) = self.packet_queue.pop_front() {
                    match self.stream.write(&packet) {
                        Ok(num_written) => {
                            if num_written < packet.len() {
                                self.packet_queue.push_front(packet[num_written..].to_owned())
                            }
                        },
                        Err(e) => {
                            self.handle_stream_error(e);
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    fn next_packet_id(&mut self) -> i32 {
        let packet_id = self.packet_id;
        self.packet_id = self.packet_id.checked_add(1).unwrap_or(1);

        packet_id
    }
}
