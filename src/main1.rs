use local_server::{get_mime_type, read_the_file};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

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
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let content = String::from_utf8_lossy(&buffer);
    let request_header: Vec<&str> = content
        .trim()
        .split("\r\n")
        .filter(|the_str| !the_str.contains("\0"))
        .collect();
    let request_path: Vec<&str> = request_header[0].split_whitespace().collect();
    let response;
    // 判断路径是否为文件

    if Path::new(&(".".to_owned() + request_path[1])).is_dir() {
        response = format!(
            "HTTP/1.1 404 OK\r\nContent-Type: {}\r\n\r\n{}",
            get_mime_type(""),
            "404 Not Found"
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let file_path = ".".to_owned() + request_path[1];
        let extension = Path::new(&file_path)
            .extension()
            .unwrap_or(OsString::from("").as_os_str())
            .to_owned();
        let file = File::open(file_path);

        match file {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer).unwrap();

                let abc = "http/2\r\nContent-Type: ".to_owned()
                    + get_mime_type(&(extension.to_str().unwrap()))
                    + "\r\n\r\n";

                let aaa = abc.as_bytes();
                let bbb = buffer.as_slice();
                let ccc = [aaa, bbb].concat();
                stream.write(ccc.as_slice()).unwrap();
                stream.flush().unwrap();
            }
            Err(err) => {
                response = format!(
                    "HTTP/1.1 404 OK\r\nContent-Type: {}\r\n\r\n{}",
                    get_mime_type(""),
                    "404 Not Found"
                );
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        };
    }
}

fn main() {
    local_server();
}
