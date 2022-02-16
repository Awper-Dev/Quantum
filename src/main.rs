use std::io::Read;
use std::net::{TcpListener, TcpStream};

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

    println!("Handled connection");

    let mut buffer = DataBuffer::from(buffer);

    let mut size = 0;
    let mut packet_id = 0;

    size = read_var_int(&mut buffer);
    packet_id = read_var_int(&mut buffer);

    if size == 0 {
        println!("Something went wrong while reading the packet.");
        return;
    }

    handle_packet(size, packet_id, buffer);
}

fn handle_packet(size: i32, packet_id: i32, buffer: DataBuffer) {
    match packet_id {
        0x00 => handle_handshake(size, buffer),
        _ => {}
    }
}

fn handle_handshake(size: i32, buffer: DataBuffer) {
    // Read: Protocol Version (VarInt), Server Address (String), Server Port (Unsigned short), Next State (VarInt Enum)
}


// https://wiki.vg/Protocol
fn read_var_int(buffer: &mut DataBuffer) -> i32 {
    let mut value: i32 = 0; // should be signed
    let mut length = 0;
    let mut current_byte: u8 = 0;

    for i in 0..5 {
        if !buffer.has_next() {
            return 0;
        }

        current_byte = buffer.next();
        value |= ((current_byte as u32 & 0x7F) << (length * 7)) as i32; // other method without casting?

        length += 1;
        if length > 5 {
            println!("VarInt too big!");
            return 0;
        }

        if (current_byte & 0x80) != 0x80 {
            break;
        }
    }

    value
}
