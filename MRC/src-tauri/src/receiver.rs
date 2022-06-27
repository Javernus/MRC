extern crate core;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use std::{thread, time};

use tauri::{Manager, Window};

use crate::database::chat::Chat;
use crate::database::group::Group;


// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub fn start_receiver(window: Window) -> () {
    // connect to socket
    let mut stream = match UnixStream::connect("/tmp/ipc.sock") {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect: {:?}", e);
            return;
        }
    };

    loop {
        thread::sleep(time::Duration::from_secs(2));
        println!("Waiting for message...");
        // window.emit("custom_event", Payload {
        //     message: "Hello from Rust!".to_string(),
        // });
        let mut message_buffer = [0; 256];
        stream.read(&mut message_buffer).expect("todo");
        let mut message = String::from_utf8(Vec::from(message_buffer)).expect("todo");

        if !message.is_empty() {
        //     // emit the `event-name` event to all webview windows on the frontend
            window.emit(
                "refetch_chat",
                Payload {message: message.to_string()}
            ).unwrap();

            let chat: Chat = Chat::new(group_id, 123456789012, "Name", message);

            database::save_chat(chat)
        //     // println!("{message}")
        }
    }
}
