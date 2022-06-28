use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use std::{thread, time};


fn main() -> std::io::Result<()> {
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
            let mut message = "testing";
            let mut message_buffer = message.as_bytes();
            stream.write_all(&mut message_buffer).expect("todo");

            println!("sending message to socket..");
            // stream.write_all(b"testing")?
        }

        let mut message_buffer = [0; 256];
        stream.read(&mut message_buffer).expect("todo");
        let mut message = String::from_utf8(Vec::from(message_buffer)).expect("todo");

        if !message.is_empty() {
            println!("{message}")
        }

    }
}
