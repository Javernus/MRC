use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use std::{thread, time};


fn main() -> std::io::Result<(UnixStream)> {
    // connect to socket
    let mut stream = match UnixStream::connect("/tmp/ipc.sock") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect: {:?}", e);
            return Ok(());
        }
    };

    loop {
        if rand::random() {
            // let mut message_buffer = [0;254];
            thread::sleep(time::Duration::from_millis(1000));
            let mut message = "testing";
            let mut message_buffer = message.as_bytes();
            stream.write_all(&mut message_buffer).expect("write to stream");

            // println!("sending message to socket..");
            // stream.write_all(b"testing")?
        }

        // TODO Read until delimiter
        let mut message_buffer = [0; 256];
        stream.read(&mut message_buffer).expect("read from stream");
        let mut message = String::from_utf8(Vec::from(message_buffer)).expect("todo");

        if !message.is_empty() {
            println!("{message}")
        }

    }
}
