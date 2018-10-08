#![allow(dead_code)]
// use bytes::{BufMut, BytesMut};
use bytes::{BytesMut};
use std::io;
use tokio::codec::{Decoder, Encoder};

use actix::Message;

/// Client request
//#[derive(Serialize, Deserialize, Debug, Message)]
//#[serde(tag = "cmd", content = "data")]
#[derive(Debug, Message)]
pub enum Request {
    /// Send message
    Message(String),
    // Ping
    // Ping,
}

/// Server response
//#[derive(Serialize, Deserialize, Debug, Message)]
//#[serde(tag = "cmd", content = "data")]
#[derive(Debug, Message)]
pub enum Response {
    // Ping
    // Ping,
    /// Message
    Message(String),
}

/// Codec for Client -> Server transport
pub struct P2PCodec;

impl Decoder for P2PCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {

        if let Some(i) = src.iter().position(|&b| b == b'\n') {
            // Remove the serialized frame from the buffer.
            let line = src.split_to(i + 1);
            let mut res = String::from_utf8(line.to_vec()).unwrap();
            res.truncate(res.len()-2);
            Ok(Some(Request::Message(res)))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for P2PCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Response, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        println!("Encoding {:?}", msg);

        // let msg = json::to_string(&msg).unwrap();
        // let msg_ref: &[u8] = msg.as_ref();

        // dst.reserve(msg_ref.len() + 2);
        // dst.put_u16_be(msg_ref.len() as u16);
        // dst.put(msg_ref);

        Ok(())
    }
}
