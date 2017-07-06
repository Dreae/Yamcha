use byteorder::{LittleEndian, WriteBytesExt};

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
pub fn parse_packet(packet: &[u8]) -> Option<(i32, String)> {
    let packet = trim_nulls(packet);
    
    unimplemented!()
}

#[inline(always)]
fn trim_nulls(buf: &[u8]) -> &[u8] {
    if let Some(first) = buf.iter().position(|b| b == 0u8) {
        buf[first..buf.iter().rposition(|b| b != 0u8) + 1]
    } else {
        &[]
    }
}