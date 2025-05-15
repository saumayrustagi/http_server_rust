use core::str;
use std::{io::Read, net::TcpStream};

const HTTP_LINE_TERMINATOR: &str = "\r\n";
const HTTP_REQUEST_TERMINATOR: &str = "\r\n\r\n";
const CONTENT_LENGTH_STRING: &str = "Content-Length: ";

pub fn handle_client(mut stream: TcpStream) -> () {
    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).unwrap();
    let s = str::from_utf8(&buffer[..n]).unwrap();
    let lines: Vec<&str> = s.split(HTTP_LINE_TERMINATOR).collect();
    let content_length = (*(lines
        .iter()
        .find(|&&s| s.starts_with(CONTENT_LENGTH_STRING))
        .unwrap()))
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();

    println!("{}", content_length);
    println!("{}:\n{:?}", s.find(HTTP_REQUEST_TERMINATOR).unwrap(), s);
}
