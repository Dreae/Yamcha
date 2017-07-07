use mio::Token;
use mio::net::TcpStream;
use std::io::{Error, ErrorKind, Write};
use std::collections::{VecDeque, HashMap};
use futures::{Future, Async, future};
use std::sync::{Arc, RwLock, Mutex};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::net::{SocketAddr, AddrParseError};

use packet::{self, PacketType};

lazy_static! {
    static ref TOKEN: AtomicUsize = AtomicUsize::new(0);
}

pub struct Connection {
    pub token: Token,
    pub stream: Mutex<TcpStream>,
    writable: AtomicBool,
    packet_queue: RwLock<VecDeque<Vec<u8>>>,
    response_map: Arc<RwLock<HashMap<i32, String>>>,
    packet_id: AtomicUsize,
}

#[derive(Clone, Debug)]
pub enum RconError {
    SocketError(ErrorKind),
    ParseFail(AddrParseError),
}

impl Connection {
    pub fn new(addr: &str, password: &str) -> Result<Arc<Connection>, RconError> {
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
        // TODO: Handle auth failures
        packet_queue.push_back(packet::build_packet(PacketType::Auth, 1, password));

        let connection = Connection {
            token: Token(TOKEN.fetch_add(1, Ordering::Relaxed)),
            stream: Mutex::new(stream),
            packet_id: AtomicUsize::new(2),
            packet_queue: RwLock::new(packet_queue),
            response_map: Arc::new(RwLock::new(HashMap::new())),
            writable: AtomicBool::new(false),
        };

        Ok(super::register(connection))
    }

    pub fn read_notify(&self, buf: Vec<Vec<u8>>) {
        for msg in buf.iter() {
            match packet::parse_packet(msg) {
                Some((packet_id, packet_type, response)) => {
                    if packet_type == PacketType::ResponseValue {
                        get_write_lock!(self.response_map).insert(packet_id, response);
                        debug!("Inserted response to packet {}", packet_id);
                    }
                },
                None => warn!("Possible junk data in stream"),
            }
        }
    }

    pub fn handle_stream_error(&self, e: Error) {
        if e.kind() != ErrorKind::WouldBlock {
            error!("Stream error {:?}", e)
        }

        if e.kind() != ErrorKind::Interrupted {
            self.writable.swap(false, Ordering::Relaxed);
        }
    }

    pub fn writable(&self) {
        self.writable.swap(true, Ordering::SeqCst);
        self.pump_queue();
    }

    pub fn notify_cmd(&self, cmd: &str) {
        get_write_lock!(self.packet_queue).push_back(packet::build_packet(PacketType::ExecCommand, 1, cmd));
    }

    // TODO: Different futures
    pub fn send_cmd(&self, cmd: &str) -> Box<Future<Item=String, Error=RconError>> {
        let packet_id = self.next_packet_id();
        get_write_lock!(self.packet_queue).push_back(packet::build_packet(PacketType::ExecCommand, packet_id, cmd));
        
        let response_map = self.response_map.clone();
        Box::new(future::poll_fn(move || {
            debug!("Checking for response to {}", packet_id);
            match get_write_lock!(response_map).remove(&packet_id) {
                Some(result) => {
                    Ok(Async::Ready(result.clone()))
                }
                None => {
                    debug!("No response to packet {} stored", packet_id);
                    Ok(Async::NotReady)
                }
            }
        }))
    }

    fn pump_queue(&self) {
        let mut queue = get_write_lock!(self.packet_queue);
        if !queue.is_empty() {
            while self.writable.load(Ordering::Acquire) {
                if let Some(packet) = queue.pop_front() {
                    match lock!(self.stream).write(&packet) {
                        Ok(num_written) => {
                            if num_written < packet.len() {
                                queue.push_front(packet[num_written..].to_owned())
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

    fn next_packet_id(&self) -> i32 {
        self.packet_id.fetch_add(1, Ordering::Relaxed) as i32
    }
}
