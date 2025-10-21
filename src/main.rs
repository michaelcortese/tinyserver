use std::fs;
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind(("::", 8080))?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut connection) => {
                handle_connection(&mut connection)?;
            }
            Err(err) => {
                panic!("{err}")
            }
        }
    }
    Ok(())
}

fn handle_connection(connection: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    connection.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));
    let index = fs::read_to_string("www/index.html")?;
    let res = format!(
        "HTTP/1.1 200 OK\r\n\
             Content-Length: {}\r\n\
             Content-Type: text/html; charset=utf-8\r\n\
             Connection: close\r\n\
             \r\n{}",
        &index.as_bytes().len(),
        index
    );

    // conclude request
    connection.write_all(res.as_bytes())?;
    connection.flush()?;
    connection.shutdown(Shutdown::Both)?;
    Ok(())
}
