use crate::data_buffer::DataBuffer;

pub fn handle_packet(data_size: i32, packet_id: i32, buffer: &mut DataBuffer) {
    match packet_id {
        0x00 => handle_handshake(data_size, buffer),
        _ => {}
    }
}

pub fn handle_handshake(data_size: i32, mut buffer: &mut DataBuffer) {
    // Read: Protocol Version (VarInt), Server Address (String), Server Port (Unsigned short), Next State (VarInt Enum)
    let mut protocol_version = 0;
    let mut server_address = String::from("No address found");
    let mut port: u16 = 0; // Unsigned short uses 2 bytes.
    let mut next_state: i32 = 0;

    buffer.read_var_int(&mut protocol_version);
    buffer.read_string(&mut server_address);
    buffer.read_unsigned_short(&mut port);
    buffer.read_var_int(&mut next_state);

    println!("Version: {}, address: {}, port: {}, next_state: {}", protocol_version, server_address, port, next_state);
}