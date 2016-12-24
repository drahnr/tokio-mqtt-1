use std::str;
use std::io::{self, BufReader, BufWriter, Read, Write, Cursor};
use std::net::SocketAddr;
use futures::BoxFuture;
use core::io::{Codec, EasyBuf, Framed, Io};
use proto::pipeline::{ClientProto};

use mqtt3::{Packet, MqttWrite, MqttRead, Error as MqttError};

//impl MqttRead for EasyBuf{}

pub struct MqttCodec;

impl Codec for MqttCodec {
    type In = Packet;
    type Out = Packet;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
        let buf = buf.as_ref();
        let mut buf = BufReader::new(buf);
        let packet = buf.read_packet().unwrap();
        Ok(Some(packet))
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        let mut stream = Cursor::new(Vec::new());
        stream.write_packet(&msg);
        stream.read_exact(buf)?;
        Ok(())
    }
}