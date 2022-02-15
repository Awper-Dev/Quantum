use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {
    setup_listener()
}

fn setup_listener() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

    while true {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Byte Array
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let mut size = 0;
    let mut packet_id = 0;

    let result = read_var_int(&mut size, &buffer, 0);
    let result = read_var_int(&mut packet_id, &buffer, (result + 1) as usize);

    if (size == 0) {
        println!("Something went wrong while reading the packet.");
        return;
    }
}

fn handle_packet(size: i32, packet_id: i32, buffer: [u8; 1024], starting_index: usize) {
    match packet_id {
        0x00 => handle_handshake(size, buffer, starting_index),
        _ => {}
    }
}

fn handle_handshake(size: i32, buffer: [u8; 1024], starting_index: usize) {
    // Read: Protocol Version (VarInt), Server Address (String), Server Port (Unsigned short), Next State (VarInt Enum)
}


// https://wiki.vg/Protocol
fn read_var_int(value: &mut i32, buffer: &[u8; 1024], starting_index: usize) -> i32 {
    //let mut value: u32 = 0; // should be signed
    let mut length = 0;
    let mut current_byte: u8 = 0;

    for i in starting_index..1023 {
        current_byte = (*buffer)[i];
        *value |= ((current_byte as u32 & 0x7F) << (length * 7)) as i32; // other method without casting?

        length += 1;
        if (length > 5) {
            println!("VarInt too big!");
            return 0;
        }

        if ((current_byte & 0x80) != 0x80) {
            break;
        }
    }
    length
}
