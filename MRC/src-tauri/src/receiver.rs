extern crate core;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use tauri::Window;

use crate::database::chat::Chat;
use crate::database;

extern crate queues;
use queues::*;

use global::Global;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

static INCOMING_QUEUE: Global<Queue<String>> = Global::new();
static OUTGOING_QUEUE: Global<Queue<String>> = Global::new();


fn read_from_socket(stream: &mut UnixStream) -> () {
    let mut buffer = [0;255];
    &stream.read(&mut buffer).expect("read from socket");

    let mut incoming_message = String::from_utf8(Vec::from(buffer)).expect("converting");

    // TODO do this with a delimeter
    if !incoming_message.is_empty() {
        (*INCOMING_QUEUE.lock_mut().unwrap()).add(incoming_message);
    }
}

// convert to send_message function
fn write_to_socket(stream: &mut UnixStream, message: String) -> (){
    let mut buffer = message.as_bytes();
    &stream.write_all(&mut buffer).expect("writing to socket");

    //TODO
}

pub fn start_client(window: Window) -> () {
    *INCOMING_QUEUE.lock_mut().unwrap() = queue![];
    *OUTGOING_QUEUE.lock_mut().unwrap() = queue![];

    //connect to socket
    let mut stream = match UnixStream::connect("/tmp/ipc.sock") {
        Ok(stream) => Ok(stream),
        Err(e) => {
            println!("Couldn't connect: {:?}", e);
            Err(e)
        }
    };

    loop {
        // check for incoming messages in socket
        read_from_socket(&mut stream.unwrap());

        // signal tauri if we have messages
        if (*INCOMING_QUEUE.lock_mut().unwrap()).size() > 0 {
            let mut incoming = (*INCOMING_QUEUE.lock_mut().unwrap()).remove().unwrap();
            let chat: Chat = Chat::new(8, 123456789012, "Name", &incoming);
            database::save_chat(&chat);

            window.emit(
                "refetch_chat",
                Payload { message: incoming.to_string() }
            ).unwrap();
        }

        // check for outgoing messages in queue
        if (*OUTGOING_QUEUE.lock_mut().unwrap()).size() > 0 {
            let mut outgoing = (*OUTGOING_QUEUE.lock_mut().unwrap()).remove().unwrap();
            write_to_socket(&mut stream.unwrap(), outgoing);
        }

    }
}
