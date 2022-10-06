use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established");
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    //println!("Request: {}",String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string("hello.html").unwrap();
    //https://developer.mozilla.org/zh-CN/docs/Web/HTTP/Headers/Transfer-Encoding
    //https://social.msdn.microsoft.com/forums/en-us/47af2197-26b4-4b9e-90e8-bfa9d5cd05b4/what-is-the-deference-between-r-n-and-rn-?forum=csharplanguage
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
    println!("response: {}",response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
