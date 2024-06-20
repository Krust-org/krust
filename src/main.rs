use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer);
    println!("Request: {}", request);

    let response = "HTTP/1.1 200 OK /r/n/r/nHello ,World!";
    stream.write(response.as_bytes());
    stream.flush();
}

fn async_request_handler(mut stream: TcpStream) {
    thread::spawn(move || {
        handle_request(stream);
    });
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000");
    println!("Server listening on 127.0.0.1:3000");

    for stream in listener.incoming() {
        async_handle_request(stream);
    }
    Ok(())
}
