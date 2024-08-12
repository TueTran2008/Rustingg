use core::panic;
//use hello::ThreadPool;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use tcp_server::ThreadPool;

fn handle_client(mut stream: TcpStream) {
    let buf_read = BufReader::new(&mut stream);
    /* let http_request: Vec<String> = buf_read
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect(); */
    let get_request: String = buf_read.lines().next().unwrap().unwrap();
    let (status_code, filepath) = if get_request == "GET / HTTP/1.1" {
        //thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK",
            "/home/tuetd/Desktop/Rustingg/tcp_server/tcp_server/src/hello.html",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND",
            "/home/tuetd/Desktop/Rustingg/tcp_server/tcp_server/src/error.html",
        )
    };
    let contents: String = fs::read_to_string(filepath).unwrap();

    let response: String = format!(
        "{} \r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listen: TcpListener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(_) => panic!("Failed to start listener"),
    };
    /*Limit the number of threads to be spwawned*/
    let pool = ThreadPool::new(4);
    for stream in listen.incoming() {
        let stream = stream.unwrap();
        println!("Connection establish");
        pool.execute(|| {
            handle_client(stream)
            //thread::spawn(|| handle_client(stream));
        });
    }
}
