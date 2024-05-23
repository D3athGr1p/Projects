use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn handle_incoming_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, response_file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /slow HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "response.html")
        }
        _ => ("HTTP/1.1 404 Not Found", "404.html"),
    };

    let response_content = fs::read_to_string(response_file).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        response_content.len(),
        response_content
    );

    println!("{response}");

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    const HOST: &str = "0.0.0.0";
    const PORT: &str = "8080";

    let pool = ThreadPool::new(100);

    let end_point = format!("{}:{}", HOST, PORT);

    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at port {}", PORT);

    for stream in listener.incoming() {
        let incoming = stream.unwrap();

        pool.execute(|| {
            handle_incoming_request(incoming);
        });
    }
}
