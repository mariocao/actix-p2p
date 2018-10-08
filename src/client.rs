extern crate actix;
extern crate futures;
extern crate tokio;

use actix::prelude::*;
use actix::{Actor, Context};

use futures::Future;
use futures::Stream;

use std::net;
use std::net::SocketAddr;

use tokio::codec::FramedRead;
use tokio::io::AsyncRead;
use tokio::net::TcpStream;
use crate::codec::P2PCodec;
use crate::session::Session;

/// Define tcp client that will connect to a server
pub struct Client {
    /// Peer address
    peer: SocketAddr
}

impl Client {
    /// Method to create a new client
    pub fn new(peer: SocketAddr) -> Self {
        Client { peer }
    }
}

#[derive(Message, Debug)]
/// Struct to hold a tcp stream and its socket addr
struct TcpConnect(pub TcpStream, pub net::SocketAddr);

/// Server handler for TcpConnect messages (built from incoming connections)
impl Handler<TcpConnect> for Client {
    /// this is response for message, which is defined by `ResponseType` trait
    /// in this case we just return unit.
    type Result = ();

    fn handle(&mut self, msg: TcpConnect, _ctx: &mut Self::Context) {
        // Create a session actor
        Session::create(move |ctx| {
            println!("Creando sesion de cliente");

            // Split tcp stream into read and write parts
            let (r, w) = msg.0.split();

            // Add message stream in session from the read part of the tcp stream (with the
            // P2P codec)
            Session::add_stream(FramedRead::new(r, P2PCodec), ctx);

            // Create the session actor and store in it the write part of the tcp stream (with the
            // P2P codec)
            Session::new(actix::io::FramedWrite::new(w, P2PCodec, ctx))
        });
    }
}

/// Make actor from `Client`
impl Actor for Client {
    /// Every actor has to provide execution `Context` in which it can run.
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Try to connect to peer
        Arbiter::spawn(TcpStream::connect(&self.peer)
            .and_then(|stream| {
                Session::create(|ctx| {
                    println!("Trying to create client session");
                    // Split tcp stream into read and write parts
                    let (r, w) = stream.split();

                    // Add message stream in session from the read part of the tcp stream (with the
                    // P2P codec)
                    Session::add_stream(FramedRead::new(r, P2PCodec), ctx);

                    // Create the session actor and store in it the write part of the tcp stream (with the
                    // P2P codec)
                    Session::new(actix::io::FramedWrite::new(w, P2PCodec, ctx))
                });

                futures::future::ok(())
            })
            .map_err(|e| println!("Cannot create client session")));
    }
}
