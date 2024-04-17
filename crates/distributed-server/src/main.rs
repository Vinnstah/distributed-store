use std::env;
use std::io::Read;
use std::net::TcpListener;

use models::messages::Message;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port: u16 = args
        .get(1)
        .expect("Failed to get port argument")
        .parse()
        .expect("Failed to parse arg as u16");

    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

    let mut buffer = [0; 1024];
    for stream in listener.incoming() {
        println!("{:#?}", stream);
        stream.expect("msg").read(&mut buffer);
        println!("{:#?}", bincode::deserialize::<Message>(&buffer))
    }
    Ok(())
}