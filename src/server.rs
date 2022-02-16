use std::io::Read;
use std::net::{TcpListener, TcpStream};

use crate::data_buffer::DataBuffer;
use crate::packet::handle_packet;

pub fn setup_listener() {
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
    let mut buffer: [u8; 4048] = [0; 4048];

    let bytes_read = stream.read(&mut buffer).unwrap();

    let mut buffer = DataBuffer::from(buffer);

    let mut size = 0;
    let mut packet_id = 0;

    buffer.read_var_int(&mut size);
    let id_size: i32 = buffer.read_var_int(&mut packet_id) as i32;

    if size == 0 {
        println!("Something went wrong while reading the packet.");
        return;
    }

    println!("handling packet with id: {}", packet_id);

    handle_packet((size as i32) - id_size, packet_id as i32, &mut buffer);
}