use mio::Token;
use mio::net::TcpStream;
use std::io::{Error, ErrorKind, Write};
use std::collections::{VecDeque, HashMap};
use std::sync::{Arc, RwLock, Mutex};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::net::{SocketAddr, AddrParseError};
use std::thread;

use packet::{self, PacketType};

lazy_static! {
    static ref TOKEN: AtomicUsize = AtomicUsize::new(0);
}

pub type RconResult = Result<String, RconError>;
pub type RconCallback = FnMut(RconResult) + Send + Sync;

pub struct Connection {
    pub token: Token,
    pub stream: Mutex<TcpStream>,
    writable: AtomicBool,
    packet_queue: RwLock<VecDeque<Vec<u8>>>,
    callback_map: Arc<RwLock<HashMap<i32, Box<RconCallback>>>>,
    packet_id: AtomicUsize,
    connected: AtomicBool,
}

#[derive(Clone, Debug)]
pub enum RconError {
    SocketError(ErrorKind),
    ParseFail(AddrParseError),
    Invalid,
}

struct CallbackResult {
    inner: RconResult,
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
            callback_map: Arc::new(RwLock::new(HashMap::new())),
            writable: AtomicBool::new(false),
            connected: AtomicBool::new(true),
        };

        Ok(super::register(connection))
    }

    pub fn read_notify(&self, buf: Vec<Vec<u8>>) {
        for msg in buf.iter() {
            let mut packets = packet::parse_packet(msg);
            
            for packet in packets.drain(..) {
                match packet {
                    Some((packet_id, packet_type, response)) => {
                        if packet_type == PacketType::ResponseValue {
                            match get_write_lock!(self.callback_map).remove(&packet_id) {
                                Some(mut cb) => cb(Ok(response)),
                                None => debug!("No callback for request {}", packet_id),
                            };
                        }
                    },
                    None => warn!("Possible junk data in stream"),
                }
            }
        }
    }

    pub fn handle_stream_error(&self, e: Error) {
        match e.kind() {
            ErrorKind::WouldBlock => {
                self.writable.store(false, Ordering::Relaxed);
             },
            ErrorKind::Interrupted => { },
            _ => {
                error!("Socket error {:?}", e);
                self.writable.store(false, Ordering::Relaxed);
                self.cancel_outstanding(e.kind());
                self.connected.store(false, Ordering::Relaxed);
            }
        }
    }

    fn cancel_outstanding(&self, kind: ErrorKind) {
        let mut callbacks = get_write_lock!(self.callback_map);
        for (_, mut cb) in callbacks.drain() {
            cb(Err(RconError::SocketError(kind)));
        }

        get_write_lock!(self.packet_queue).clear();
    }

    pub fn writable(&self) {
        self.writable.store(true, Ordering::SeqCst);
        self.pump_queue();
    }

    pub fn notify_cmd(&self, cmd: &str) {
        self._send_cmd(cmd, None);
    }

    pub fn send_cmd_sync(&self, cmd: &str) -> RconResult {
        let res: Arc<Mutex<CallbackResult>> = Arc::new(Mutex::new(CallbackResult {
            inner: Err(RconError::Invalid)
        }));
        
        let handle = thread::current();
        let return_res = res.clone();
        self.send_cmd(cmd, Box::new(move |res| {
            {
                lock!(return_res).inner = res;
            }
            handle.unpark();
        }));
        thread::park();
        
        // Should be safe, res is only locked in the callback
        // and is released before this thread is unparked
        match Arc::try_unwrap(res) {
            Ok(lock) => {
                match lock.into_inner().unwrap().inner {
                    Ok(res) => Ok(res),
                    Err(e) => Err(e),
                }
            },
            Err(_) => {
                unreachable!()
            }
        }
    }

    pub fn send_cmd(&self, cmd: &str, cb: Box<RconCallback>) {
        self._send_cmd(cmd, Some(cb));
    } 

    fn _send_cmd(&self, cmd: &str, cb: Option<Box<RconCallback>>) {
        if !self.connected.load(Ordering::Acquire) {
            if let Some(mut cb) = cb {
                cb(Err(RconError::SocketError(ErrorKind::NotConnected)));
                return;
            }
        }

        let packet_id = self.next_packet_id();
        get_write_lock!(self.packet_queue).push_back(packet::build_packet(PacketType::ExecCommand, packet_id, cmd));

        if let Some(cb) = cb {
            get_write_lock!(self.callback_map).insert(packet_id, cb);
        }

        self.pump_queue();
    }

    fn pump_queue(&self) {
        if self.writable.load(Ordering::SeqCst) {
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
    }

    fn next_packet_id(&self) -> i32 {
        self.packet_id.fetch_add(1, Ordering::Relaxed) as i32
    }
}
