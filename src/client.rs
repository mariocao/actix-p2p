extern crate actix;
extern crate futures;
extern crate tokio;

use actix::actors::resolver::{ConnectAddr, Resolver};
use actix::prelude::*;
use actix::{Actor, Context};

use std::net::SocketAddr;

use crate::codec::P2PCodec;
use crate::session::Session;
use tokio::codec::FramedRead;
use tokio::io::AsyncRead;

/// Define tcp client that will connect to a server
pub struct Client {
    /// Peer address
    peer: SocketAddr,
}

impl Client {
    /// Method to create a new client
    pub fn new(peer: SocketAddr) -> Self {
        Client { peer }
    }
}

/// Make actor from `Client`
impl Actor for Client {
    /// Every actor has to provide execution `Context` in which it can run.
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Connecting to server...");

        Resolver::from_registry()
            .send(ConnectAddr(self.peer))
            .into_actor(self)
            .map(|res, act, ctx| match res {
                Ok(stream) => {
                    println!("Connected to server `{}`", act.peer);
                    Session::create(move |ctx| {
                        // Split tcp stream into read and write parts
                        let (r, w) = stream.split();

                        // Add message stream in session from the read part of the tcp stream (with the
                        // P2P codec)
                        Session::add_stream(FramedRead::new(r, P2PCodec), ctx);

                        // Create the session actor and store in it the write part of the tcp stream (with the
                        // P2P codec)
                        Session::new(actix::io::FramedWrite::new(w, P2PCodec, ctx))
                    });
                }
                Err(err) => {
                    println!("Cannot connect to server: {}", err);
                    ctx.stop();
                }
            })
            .map_err(|err, act, ctx| {
                println!("Cannot connect to server `{}`: {}", act.peer, err);
                ctx.stop();
            })
            .wait(ctx);
    }
}
