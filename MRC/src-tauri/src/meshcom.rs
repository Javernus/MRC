extern crate systemstat;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use systemstat::{System, Platform};

fn main() {
    let sys = System::new();
    let mut stream = UnixStream::connect("../../../interface/ipc.sock").unwrap();

    let (tx, rx) = channel();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(3));

            match sys.cpu_temp() {
                Ok(cpu_temp) => {
                    tx.send(format!("Temp is: {}", cpu_temp));
                },
                Err(x) => {
                    tx.send("error".to_string());
                }
            };
        }

    });

    loop {
        let _ = rx
            .try_recv()
            .map(|reply| stream.write_all(reply.as_bytes()));
    }
}