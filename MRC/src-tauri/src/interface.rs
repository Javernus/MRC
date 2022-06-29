extern crate core;
use tauri::Window;
use crate::database::chat::Chat;
use crate::database;
extern crate queues;
use queues::*;

extern crate global;
use global::Global;

use tokio::io::Interest;
use tokio::net::UnixStream;
use std::error::Error;
use std::io;
use std::str;
use std::time::Duration;
use tokio::time::sleep;


// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// TODO implement custom structs
static OUTGOING_QUEUE: Global<Queue<String>> = Global::new();

fn send_message(message: String) -> () {
    (*OUTGOING_QUEUE.lock_mut().unwrap()).add(message).expect("adding message");
}

// TODO make this its own function
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    *OUTGOING_QUEUE.lock_mut().unwrap() = queue![];

    let stream = UnixStream::connect("/tmp/ipc.sock").await?;

    loop {
        let _ = sleep(Duration::from_millis(1000)).await;
        let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;

        if rand::random() {
            send_message("Hello world".parse().unwrap());
        }

        if ready.is_readable() {
            let mut data = vec![0; 255];

            match stream.try_read(&mut data) {
                Ok(_) => {}
                Err(ref e)  if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            };

            let incoming_message = str::from_utf8(&data).unwrap();

            if !incoming_message.is_empty() {
                println!("read {incoming_message} from socket");

                let chat: Chat = Chat::new(8, 123456789012, "Name", &incoming);
                database::save_chat(&chat);

                window.emit(
                    "refetch_chat",
                    Payload { message: incoming.to_string() }
                ).unwrap();
            }
        }

        if ready.is_writable() {
            if (*OUTGOING_QUEUE.lock_mut().unwrap()).size() > 0 {
                let outgoing_message = (*OUTGOING_QUEUE.lock_mut().unwrap()).remove().unwrap();


                match stream.try_write((outgoing_message + "\n").as_bytes()) {
                    Ok(_) => {}
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }

        }
    }
}
