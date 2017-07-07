use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};

#[derive(PartialEq)]
pub enum PacketType {
    Auth,
    AuthResponse,
    ExecCommand,
    ResponseValue,
    Invalid,
}

impl PacketType {
    fn to_i32(self) -> i32 {
        match self {
            PacketType::Auth => 3,
            PacketType::AuthResponse => 2,
            PacketType::ExecCommand => 2,
            PacketType::ResponseValue => 0,
            PacketType::Invalid => -1,
        }
    }

    pub fn from_i32(n: i32, is_response: bool) -> PacketType {
        match n {
            3 => PacketType::Auth,
            2 if is_response => PacketType::AuthResponse,
            2 => PacketType::ExecCommand,
            0 => PacketType::ResponseValue,
            _ => PacketType::Invalid,
        }
    }
}


#[inline(always)]
pub fn build_packet(packet_type: PacketType, packet_id: i32, body: &str) -> Vec<u8> {
    let mut pkt = Vec::new();

    pkt.write_i32::<LittleEndian>((body.len() + 10) as i32).unwrap();
    pkt.write_i32::<LittleEndian>(packet_id).unwrap();
    pkt.write_i32::<LittleEndian>(packet_type.to_i32()).unwrap();
    pkt.extend(body.as_bytes());
    pkt.push(0u8);
    pkt.push(0u8);

    pkt
}

#[inline(always)]
pub fn parse_packet(mut packet: &[u8]) -> Option<(i32, PacketType, String)> {
    if packet.len() < 14 {
        return None;
    }

    let packet_len = packet.read_i32::<LittleEndian>().unwrap();
    let packet_id = packet.read_i32::<LittleEndian>().unwrap();
    let packet_type = packet.read_i32::<LittleEndian>().unwrap();
    
    let body = String::from_utf8_lossy(&packet[0..packet_len as usize - 10]);
    
    Some((packet_id, PacketType::from_i32(packet_type, true), (*body).to_owned()))
}