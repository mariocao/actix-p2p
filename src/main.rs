extern crate actix;
extern crate bytes;
extern crate futures;
extern crate tokio;

use actix::{Actor, System};
use std::env;

mod client;
mod codec;
mod server;
mod session;

use crate::client::Client;
use crate::server::Server;

/// Default server port
const DEFAULT_SERVER_PORT: u16 = 12345;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check number of arguments
    let port: u16 = match args.len() {
        // one argument passed
        2 => args[1].parse().unwrap_or(DEFAULT_SERVER_PORT),

        // no arguments passed
        _ => DEFAULT_SERVER_PORT,
    };

    // Init system
    let system = System::new("p2p");

    // Init session manager
    //let session_manager = SessionManager::default();

    // Start session manager
    //let session_manager_addr = session_manager.start();

    // Init server actor
    let server = Server::new(port);

    // Start server actor
    let _addr = server.start();

    // Init client actor
    let peer_addr = "127.0.0.1:50000".parse().unwrap();
    let client = Client::new(peer_addr);

    // Start client actor
    let _addr = client.start();

    // Run system
    system.run();
}
