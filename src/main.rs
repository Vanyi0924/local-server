use std::fs::{read, read_dir};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use regex::Regex;

fn read_the_dir() {
    let dir = read_dir(".").unwrap();
    dir.for_each(|dir_result| match dir_result {
        Ok(entry) => {
            println!("{:?}", entry.file_name());
            println!("{:?}", entry.file_type());
            println!("{:?}", entry.path());
            println!("{:?}", entry.metadata());
            println!("------");
        }
        Err(err) => println!("{}", err),
    })
}

fn local_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();
    let content = String::from_utf8_lossy(&buffer);
    let request_header: Vec<&str> = content
        .trim()
        .split("\r\n")
        .filter(|the_str| !the_str.contains("\0"))
        .collect();
    // let request_path = data.as_str();
    let request_path: Vec<&str> = request_header[0].split_whitespace().collect();
    let response_data = read_the_file(&(".".to_owned() + request_path[1]));
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", response_data);
    stream.write(response.as_bytes()).unwrap();
    // // flush 会等待并阻塞程序执行直到所有字节都被写入连接中
    stream.flush().unwrap();
}

fn read_the_file(path: &str) -> String {
    println!("{}", path);
    let content = read(path).unwrap();
    String::from_utf8_lossy(&content).to_string()
}

fn main() {
    // read_the_dir();
    // read_the_file();
    local_server();
}
