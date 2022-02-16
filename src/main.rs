use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;

// Move into its own file
// Make these private for better encapsulation
struct DataBuffer {
    buffer: [u8; 1024],
    next_read: usize,
}

impl DataBuffer {
    fn from(buffer: [u8; 1024]) -> DataBuffer {
        DataBuffer {
            buffer,
            next_read: 0
        }
    }

    fn has_next(&self) -> bool {
        self.next_read < self.buffer.len()
    }

    fn next(&mut self) -> u8 {
        self.next_read += 1;
        self.buffer[self.next_read - 1]
    }
}

fn main() {
    setup_listener()
}

fn setup_listener() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

    loop {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Byte Array
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let mut buffer = DataBuffer::from(buffer);

    let mut size = 0;
    let mut packet_id = 0;

    read_var_int(&mut size, &mut buffer);
    let id_size: i32 = read_var_int(&mut packet_id, &mut buffer) as i32;

    if size == 0 {
        println!("Something went wrong while reading the packet.");
        return;
    }

    handle_packet((size as i32) - id_size, packet_id as i32, buffer);
}

fn handle_packet(data_size: i32, packet_id: i32, buffer: DataBuffer) {
    match packet_id {
        0x00 => handle_handshake(data_size, buffer),
        _ => {}
    }
}

fn handle_handshake(data_size: i32, mut buffer: DataBuffer) {
    // Read: Protocol Version (VarInt), Server Address (String), Server Port (Unsigned short), Next State (VarInt Enum)
    let mut protocol_version = 0;
    let mut server_address = String::from("No address found");
    let mut port: u16 = 0; // Unsigned short uses 2 bytes.
    let mut next_state: u32 = 0;

    read_var_int(&mut protocol_version, &mut buffer);
    read_string(&mut server_address, &mut buffer);
    read_unsigned_short(&mut port, &mut buffer);
    read_var_int(&mut next_state, &mut buffer);

    println!("Version: {}, address: {}, port: {}, next_state: {}", protocol_version, server_address, port, next_state);
}

// Big endian
fn read_unsigned_short(value: &mut u16, buffer: &mut DataBuffer) -> u32 {
    *value = 0;

    *value |= ((buffer.next() as u16) << 8);
    *value |= buffer.next() as u16;

    2 // Read 2 bytes
}

fn read_string(value: &mut String, buffer: &mut DataBuffer) -> u32 {
    let mut bytes_read = 0;
    let mut string_size = 0;
    bytes_read += read_var_int(&mut string_size, buffer);

    let mut unicode_vector = Vec::new();

    for i in 0..string_size {
        unicode_vector.push(buffer.next());
    }

    let string = str::from_utf8_mut(&mut unicode_vector);

    match string {
        Ok(result) => *value = result.parse().unwrap(), // From slice to string
        Err(error) => *value = String::from(error.to_string()),
    }

    bytes_read += string_size; // Might not be accurate if string wasn't valid
    bytes_read
}

// https://wiki.vg/Protocol
// Little endian
fn read_var_int(value: &mut u32, buffer: &mut DataBuffer) -> u32 {
    let mut length = 0;
    let mut current_byte: u8 = 0;

    for i in 0..5 {
        if !buffer.has_next() {
            return 0;
        }

        current_byte = buffer.next();
        *value |= ((current_byte as u32 & 0x7F) << (length * 7)); // other method without casting?

        length += 1;
        if length > 5 {
            println!("VarInt too big!");
            return 0;
        }

        if (current_byte & 0x80) != 0x80 {
            break;
        }
    }

    length
}
