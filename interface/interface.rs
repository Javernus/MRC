use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use std::{thread, time};


fn main() -> std::io::Result<()> {
    // connect to socket
    let mut stream = match UnixStream::connect("ipc.sock") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect: {:?}", e);
            return Ok(());
        }
    };

    loop {
        let mut message_buffer = [0; 256];
        stream.read(&mut message_buffer).expect("todo");
        let mut message = String::from_utf8(Vec::from(message_buffer)).expect("todo");

        if !message.is_empty() {
            println!("{message}")
        }

    }

    Ok(())
}
