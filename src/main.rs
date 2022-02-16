use crate::server::setup_listener;

mod server;
mod data_buffer;
mod packet;

fn main() {
    setup_listener();
}
