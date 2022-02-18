use crate::data_buffer::DataBuffer;

pub fn handle_packet(data_size: i32, packet_id: i32, buffer: &mut DataBuffer) -> Option<DataBuffer> {
    let mut response = None;

    match packet_id {
        0x00 => response = handle_handshake(data_size, buffer),
        0x01 => response = handle_ping(data_size, buffer),
        _ => {}
    }

    return response;
}

pub fn handle_ping(_data_size: i32, buffer: &mut DataBuffer) -> Option<DataBuffer> {
    // Copies the buffer right now, maybe we can do this better
    return Some(DataBuffer::from(buffer.buffer.to_vec()));
}

// https://wiki.vg/Server_List_Ping
pub fn handle_handshake(data_size: i32, buffer: &mut DataBuffer) -> Option<DataBuffer> {
    println!("handling a handshake packet...");

    // actually check the connection state instead of the data_size. This works for now
    if data_size == 0 {
        // Request packet, we should respond
        println!("Sending response...");
        return Some(write_ping_response());
    }

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

    return None;
}

fn write_ping_response() -> DataBuffer {

    let response = "{\"description\":{\"text\":\"quatum is cool\"},\"players\":{\"max\":999999,\"online\":16},\"version\":{\"name\":\"1.18.1\",\"protocol\":757}}";
    let mut buffer = DataBuffer::from(response.as_bytes().to_vec()); // response buffer

    // varint id packet
    let mut id_buf = DataBuffer::new();
    id_buf.write_var_int(0);

    // varint total length of packet
    let mut total_length_buf = DataBuffer::new();
    let total_length = 2 + (buffer.buffer.len() + total_length_buf.buffer.len()) as i32;
    total_length_buf.write_var_int(total_length);

    // varint different length (doesn't seem to be documented, but works for now)
    let mut length_buf = DataBuffer::new();
    let length = (buffer.buffer.len() + total_length_buf.buffer.len()) as i32 - 1;
    length_buf.write_var_int(length);

    // full packet buffer
    let mut full_buf = Vec::with_capacity((total_length + length + id_buf.buffer.len() as i32 + buffer.buffer.len() as i32) as usize);
    full_buf.append(&mut total_length_buf.buffer);
    full_buf.append(&mut id_buf.buffer);
    full_buf.append(&mut length_buf.buffer);
    full_buf.append(&mut buffer.buffer);

    return DataBuffer::from(full_buf);
}
