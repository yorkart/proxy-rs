use std::net::SocketAddr;
use std::io;
use std::io::Read;

use futures::{Future, Stream, future};

use bytes::{BytesMut, BufMut};

use tokio_core::net::{TcpListener, TcpStream};

use tokio_core::reactor::Core;
use tokio_core::io::Io;
use tokio_io::codec::{Encoder, Decoder};
use tokio_io::AsyncRead;

pub fn serve() {
    let core = Core::new().unwrap();
    let handle = core.handle();

    let addr = "0.0.0.0:36366".parse::<SocketAddr>().unwrap();
    let listen = TcpListener::bind(&addr, &handle).unwrap();
    let addr = listen.local_addr().unwrap();

    println!("addr: {:?}", addr);

    let srv = listen.incoming().map(|(mut socket, addr)| {
//        let socket = future::ok(socket);
//        socket.and_then()

        let (sink, stream) = socket.framed(LineCodec).split();
        let a = sink.send_all(stream).map(|_| ()).map_err(|_| ());
        handle.spawn(a);
        Ok(())

    });

    handle.spawn(srv.map_err(|e| panic!("srv error: {}", e)));

}

pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<BytesMut>, io::Error> {
        match buf.iter().position(|&b| b == b'\n') {
            Some(i) => Ok(Some(buf.split_to(i + 1).into())),
            None => Ok(None),
        }
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        if buf.len() == 0 {
            Ok(None)
        } else {
            let amt = buf.len();
            Ok(Some(buf.split_to(amt)))
        }
    }
}

impl Encoder for LineCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn encode(&mut self, item: BytesMut, into: &mut BytesMut) -> io::Result<()> {
        into.put(&item[..]);
        Ok(())
    }
}