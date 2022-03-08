use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

extern crate asyncio;
use asyncio::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let job_definitions = vec![
        JobDefinition::new(vec![
            Box::new(move || {
                println!("Hello from task 1");
            }),
            Box::new(move || {
                println!("Hello from task 2");
            }),
        ]),
        JobDefinition::new(vec![
            || {
                println!("hello from job 1");
            },
            || {
                println!("hello from job 2");
            },
        ]),
    ];

    let manager = ThreadManager::new(job_definitions);

    static QUICK_PATH: bool = false;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        manager.execute(|| handle_connection(stream));
    }

    println!("Shutting Down");
}


fn read_stream(stream: &mut TcpStream) {
    
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
}

fn read_file () {
    let slash = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // println!("---------------------------------------------------------------");
    // println!("\nRequest: {}\n", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(slash) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
}

fn write_file () {
    // write file
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // println!("Response: {}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("\n---------------------------------------------------------------\n\n");
}

// jobs can be designated async or not
// figure out how to have data that persists for entire job
// figure out how to provide a routes structure
fn handle_connection(mut stream: TcpStream) {

}
