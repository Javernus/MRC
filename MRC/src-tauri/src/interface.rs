extern crate core;
use tauri::Window;
use crate::database::chat::Chat;
use crate::database;
extern crate queues;
extern crate time;
use queues::*;

use crate::find_correct_group;

extern crate global;
use global::Global;

use tokio::io::Interest;
use tokio::net::UnixStream;
use std::error::Error;
use std::io;
use std::str;
use std::time::Duration;
use time::get_time;
use tokio::time::sleep;


// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// TODO implement custom structs
static OUTGOING_QUEUE: Global<Queue<String>> = Global::new();

pub fn send_message(message: String) -> () {
    (*OUTGOING_QUEUE.lock_mut().unwrap()).add(message).expect("adding message");
}

pub async fn start_client(window: Window) -> Result<(), Box<dyn Error>> {
    *OUTGOING_QUEUE.lock_mut().unwrap() = queue![];

    println!("trying to connect to socket..");
    let stream = UnixStream::connect("/tmp/ipc.sock").await?;

    loop {
        let _ = sleep(Duration::from_millis(5000)).await;
        let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;

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

                let correct_group = find_correct_group(incoming_message.to_string());

                if correct_group.0 != -1 {
                    let time = get_time();

                    let ftime: i64 = i64::try_from(time.sec * 1000).unwrap() + i64::try_from(time.nsec / 1000).unwrap();

                    let chat: Chat = Chat::new(correct_group.0, ftime, &correct_group.1, &correct_group.2);

                    if database::append_chat(&chat).is_err() {
                        // TODO: throw error perhaps?
                    }
                }
                // let chat: Chat = Chat::new(8, 123456789012, "Name", &incoming_message);

                window.emit(
                    "refetch_chat",
                    Payload { message: incoming_message.to_string() }
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
