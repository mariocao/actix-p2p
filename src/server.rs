extern crate actix;
extern crate futures;
extern crate tokio;

use std::net;
use std::str::FromStr;

use actix::prelude::*;
use actix::{Actor, Context};
use futures::Stream;

use tokio::codec::FramedRead;
use tokio::io::AsyncRead;
use tokio::net::{TcpListener, TcpStream};

use crate::codec::P2PCodec;
use crate::session::Session;

/// Define tcp server that will accept incoming tcp connection and create
/// chat actors.
pub struct Server {
    pub port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }
}

#[derive(Message, Debug)]
struct TcpConnect(pub TcpStream, pub net::SocketAddr);

/// Handle stream of TcpStream's
impl Handler<TcpConnect> for Server {
    /// this is response for message, which is defined by `ResponseType` trait
    /// in this case we just return unit.
    type Result = ();

    fn handle(&mut self, msg: TcpConnect, _ctx: &mut Self::Context) {
        // let server = self.chat.clone();
        Session::create(move |ctx| {
            let (r, w) = msg.0.split();
            Session::add_stream(FramedRead::new(r, P2PCodec), ctx);
            Session::new(actix::io::FramedWrite::new(w, P2PCodec, ctx))
        });
    }
}

/// Make actor from `Server`
impl Actor for Server {
    /// Every actor has to provide execution `Context` in which it can run.
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = net::SocketAddr::from_str("127.0.0.1:12345").unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        ctx.add_message_stream(listener.incoming().map_err(|_| ()).map(|stream| {
            let addr = stream.peer_addr().unwrap();
            TcpConnect(stream, addr)
        }));

        println!("P2P server has been started at {:?}", addr);
    }
}
