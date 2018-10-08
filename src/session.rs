extern crate actix;
extern crate futures;
extern crate tokio;

use actix::StreamHandler;
use std::io;

use actix::Actor;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;

use crate::codec::{P2PCodec, Request};

pub struct Session {
    /// unique session id
    id: usize,
    // Framed wrapper
    _framed: actix::io::FramedWrite<WriteHalf<TcpStream>, P2PCodec>,
}

/// Helper methods
impl Session {
    pub fn new(_framed: actix::io::FramedWrite<WriteHalf<TcpStream>, P2PCodec>) -> Session {
        Session { id: 0, _framed }
    }
}

impl Actor for Session {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        // self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // self.addr
        //     .send(server::Connect {
        //         addr: ctx.address(),
        //     })
        //     .into_actor(self)
        //     .then(|res, act, ctx| {
        //         match res {
        //             Ok(res) => act.id = res,
        //             // something is wrong with chat server
        //             _ => ctx.stop(),
        //         }
        //         actix::fut::ok(())
        //     })
        //     .wait(ctx);
    }

    // fn stopping(&mut self, _: &mut Self::Context) -> Running {
    //     // notify chat server
    //     // self.addr.do_send(server::Disconnect { id: self.id });
    //     // Running::Stop
    // }
}

impl actix::io::WriteHandler<io::Error> for Session {}

/// To use `Framed` with an actor, we have to implement `StreamHandler` trait
impl StreamHandler<Request, io::Error> for Session {
    /// This is main event loop for client requests
    fn handle(&mut self, msg: Request, _ctx: &mut Self::Context) {
        match msg {
            Request::Message(message) => {
                // send message to chat server
                println!("Peer {} message received `{}`", self.id, message);
                // self.addr.do_send(server::Message {
                //     id: self.id,
                //     msg: message,
                //     room: self.room.clone(),
                // })
            }
            // we update heartbeat time on ping from peer
            // Request::Ping => self.hb = Instant::now(),
        }
    }
}
