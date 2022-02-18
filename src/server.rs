use std::io::{Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};

use crate::data_buffer::DataBuffer;
use crate::packet::handle_packet;

pub fn setup_listener() {
    let listener = TcpListener::bind("0.0.0.0:25565").unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            println!("Opened new stream");
            handle_connection(&stream);
        }
    }
}

fn handle_connection(mut stream: &TcpStream) -> () {
    let mut reader = BufReader::new(stream);

    loop {
        let buffer = reader.fill_buf().unwrap().to_vec();
        if buffer.len() == 0 {
            // stream closed
            break;
        }

        let mut buffer = DataBuffer::from(buffer);

        let mut size = 0;
        let mut packet_id = 0;

        let size_size = buffer.read_var_int(&mut size);
        let id_size: i32 = buffer.read_var_int(&mut packet_id) as i32;

        if size == 0 {
            break;
        }

        println!("Got client packet with id: {}", packet_id);

        // DEBUG INFO
        //println!("size is {}, sum is {}, bytes_read is {}", size, size_size + size as u32, buffer.buffer.len());

        reader.consume((size_size + size as u32) as usize);

        let response_option = handle_packet((size as i32) - id_size, packet_id as i32, &mut buffer);

        if let Some(data_buffer) = response_option {
            stream.write_all(&data_buffer.buffer).unwrap();
            stream.flush().expect("Failed to flush stream");
        }

        println!("Done handling client packet!");
    }

    println!("Stream closed");
}