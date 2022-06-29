extern crate core;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
// use tauri::Window;
// use crate::database::chat::Chat;
// use crate::database;
extern crate queues;
use queues::*;

extern crate global;
use global::Global;

// the payload type must implement `Serialize` and `Clone`.
// #[derive(Clone, serde::Serialize)]
// struct Payload {
//     message: String,
// }

// TODO implement custom structs
static INCOMING_QUEUE: Global<Queue<String>> = Global::new();
static OUTGOING_QUEUE: Global<Queue<String>> = Global::new();


fn read_from_socket(mut stream: &UnixStream) -> () {
    println!("trying to read from socket..");
    let mut buffer = [0;255];
    let _ = &stream.read(&mut buffer).expect("read from socket");

    let incoming_message = String::from_utf8(Vec::from(buffer)).expect("converting");

    // TODO do this with a delimeter
    if !incoming_message.is_empty() {
        println!("read {incoming_message} from socket");
        (*INCOMING_QUEUE.lock_mut().unwrap()).add(incoming_message).expect("adding to queue");
    }
}

// convert to send_message function
fn write_to_socket(mut stream: &UnixStream, message: String) -> (){
    let mut buffer = message.as_bytes();
    stream.write_all(&mut buffer).expect("writing to socket");

    //TODO
}

fn send_message(message: String) -> (){
    (*OUTGOING_QUEUE.lock_mut().unwrap()).add(message).expect("adding message");
}

// TODO make this its own function
#[tokio::main]
async fn main() -> () {
    *INCOMING_QUEUE.lock_mut().unwrap() = queue![];
    *OUTGOING_QUEUE.lock_mut().unwrap() = queue![];

    //connect to socket
    let stream = match UnixStream::connect("/tmp/ipc.sock") {
        Ok(stream) => Ok(stream),
        Err(e) => {
            println!("Couldn't connect: {:?}", e);
            Err(e)
        }
    };

    loop {
        // check for incoming messages in socket
        read_from_socket(stream.as_ref().unwrap());

        if rand::random() {
            println!("sending test message..");
            send_message("testMessage".parse().unwrap())
        }

        // signal tauri if we have messages
        if (*INCOMING_QUEUE.lock_mut().unwrap()).size() > 0 {
            let incoming = (*INCOMING_QUEUE.lock_mut().unwrap()).remove().unwrap();
            println!("we have an incoming message: {incoming}");

            // let chat: Chat = Chat::new(8, 123456789012, "Name", &incoming);
            // database::save_chat(&chat);
            //
            // window.emit(
            //     "refetch_chat",
            //     Payload { message: incoming.to_string() }
            // ).unwrap();
        }

        // check for outgoing messages in queue
        if (*OUTGOING_QUEUE.lock_mut().unwrap()).size() > 0 {
            println!("we have an outgoing message");
            let outgoing = (*OUTGOING_QUEUE.lock_mut().unwrap()).remove().unwrap();
            write_to_socket( stream.as_ref().unwrap(), outgoing);
        }

    }
}
