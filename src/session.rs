extern crate actix;
extern crate futures;
extern crate tokio;

use actix::Actor;
use actix::StreamHandler;

use std::io;

use tokio::io::WriteHalf;
use tokio::net::TcpStream;

use crate::codec::{P2PCodec, Request};

pub struct Session {
    /// Unique session id
    id: usize,
    /// Framed wrapper to send messages through the TCP connection
    _framed: actix::io::FramedWrite<WriteHalf<TcpStream>, P2PCodec>,
}

/// Session helper methods
impl Session {
    /// Method to create a new session
    pub fn new(_framed: actix::io::FramedWrite<WriteHalf<TcpStream>, P2PCodec>) -> Session {
        Session { id: 0, _framed }
    }
}

/// Implement actor trait for Session
impl Actor for Session {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
    }
}

/// Implement write handler for Session
impl actix::io::WriteHandler<io::Error> for Session {}

/// Implement `StreamHandler`trait in order to use `Framed` with an actor
impl StreamHandler<Request, io::Error> for Session {
    /// This is main event loop for client requests
    fn handle(&mut self, msg: Request, _ctx: &mut Self::Context) {
        // Handler different types of requests
        match msg {
            Request::Message(message) => {
                println!("Peer {} message received `{}`", self.id, message);
            }
        }
    }
}
