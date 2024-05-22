use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn handle_incoming_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let status_line = "HTTP/1.1";
    let mut response_content: String = String::new();
    let mut response_content_length = 0;
    let mut response = String::new();

    if request_line == "GET / HTTP/1.1" {
        response_content = fs::read_to_string("index.html").unwrap();
        response_content_length = response_content.len();
        response = format!(
            "{} 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            status_line, response_content_length, response_content
        );
    } else {
        response_content = fs::read_to_string("404.html").unwrap();
        response_content_length = response_content.len();
        response = format!(
            "{} 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
            status_line, response_content_length, response_content
        );
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    const HOST: &str = "0.0.0.0";
    const PORT: &str = "8080";

    let end_point = format!("{}:{}", HOST, PORT);

    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at port {}", PORT);

    for stream in listener.incoming() {
        let incoming = stream.unwrap();

        handle_incoming_request(incoming);
    }
}
