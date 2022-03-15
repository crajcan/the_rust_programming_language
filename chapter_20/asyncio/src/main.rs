use std::fs::File;
use std::io::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, Duration};


#[tokio::main]
async fn main() {
    let listener: tokio::net::TcpListener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    println!("handling connection");
    let mut buffer = [0; 512];
    stream.read(&mut buffer).await.unwrap();

    let slash = b"GET / HTTP/1.1\r\n";
    let sleep_route = b"GET /sleep HTTP/1.1\r\n";

    println!("---------------------------------------------------------------");
    println!("\nRequest: {}\n", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(slash) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep_route) {
        sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    println!("Response: {}", response);

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
 
    let x: std::ops::Range<i32> = 1..10;
    let foo = "this is a foo";

    for i in x {
        println!("i: {}", i);
    }
    println!("\n---------------------------------------------------------------\n\n");
}
