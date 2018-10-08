extern crate actix;

extern crate byteorder;
extern crate bytes;
extern crate serde;

use actix::{Actor, System};

mod codec;
mod server;
mod session;

use crate::server::Server;

fn main() {
    let system = System::new("p2p");

    let server = Server::new(12345);
    let _addr = server.start();

    system.run();
}
