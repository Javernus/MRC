use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use std::{thread, time};

use tauri::Manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}


pub fn start_receiver() -> () {
    // connect to socket
    let mut stream = match UnixStream::connect("/tmp/ipc.sock") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect: {:?}", e);
            return;
        }
    };

    loop {
        let mut message_buffer = [0; 256];
        stream.read(&mut message_buffer).expect("todo");
        let mut message = String::from_utf8(Vec::from(message_buffer)).expect("todo");

        if !message.is_empty() {
            // emit the `event-name` event to all webview windows on the frontend
            app.emit_all(
                "message_event",
                Payload { message: message.into() }
            ).unwrap();
            // println!("{message}")
        }
    }
}
