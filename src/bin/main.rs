use std::fs;
use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;

use concurrent::ThreadPool;
fn main() {
    let tcp = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in tcp.incoming(){
        let stream  = stream.unwrap();
        
        pool.execute(||{
            handleconnection(stream);
        });

    }
}

fn handleconnection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();


    let resp = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(resp) { 
        ("HTTP/1.1 200 OK","index.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",status_line,contents.len(),contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();           
}