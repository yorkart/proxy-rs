extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

mod server;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    server::serve();

    thread::sleep(Duration::new(180, 0));
}
