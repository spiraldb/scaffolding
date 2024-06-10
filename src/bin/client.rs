use std::net::TcpStream;

// Remove.
#[allow(unused_mut)]
#[allow(unused_variables)]
// Run with `cargo run --bin client`.
fn main() {
    let mut args = std::env::args();
    _ = args.next();

    let mut stream = TcpStream::connect("127.0.0.1:4242").unwrap();

    println!("Connected to the server.");
    todo!()
}
