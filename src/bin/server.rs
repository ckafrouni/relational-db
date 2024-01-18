use relational_db::database::Database;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

static INDEX_HTML: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<!DOCTYPE html><html><head><title>Relational DB</title></head><body><h1>Relational DB</h1></body></html>";

fn handle_client(mut stream: TcpStream, db: &mut Database) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            // let query = String::from_utf8_lossy(&buffer);
            // Assuming execute_query is a method that takes a query,
            // executes it, and returns the result
            // let result = db.execute(query.into());
            // stream.write_all(result.as_bytes()).unwrap();
            stream.write_all(INDEX_HTML).unwrap();
        }
        Err(e) => eprintln!("Failed to read from stream: {}", e),
    }
}

fn main() {
    let mut db = Database::default();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream, &mut db);
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
