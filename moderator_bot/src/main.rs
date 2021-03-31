use std::net::{TcpListener};
use std::fs;
// application/toml
fn main() {
    println!("{:?}", load_config("./dummy.toml"));
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
        // handle_connection(_stream)
    }
}
fn load_config(filename:&str)->String{
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents
}
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];

//     stream.read(&mut buffer).unwrap();

//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
// }
// fn setup_main_handler(){}
