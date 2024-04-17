use std::env;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let port: u16 = args
        .get(1)
        .expect("Failed to get port argument")
        .parse()
        .expect("Failed to parse arg as u16");

    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

    for stream in listener.incoming() {
        println!("{:#?}", stream)
    }
    Ok(())
}
