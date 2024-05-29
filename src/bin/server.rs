use std::net::TcpListener;

// Remove.
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let mut args = std::env::args();
    _ = args.next();

    let listener = TcpListener::bind("127.0.0.1:4242").unwrap();
    for stream in listener.incoming() {
        println!("Client connected.");
        todo!()
    }
}
