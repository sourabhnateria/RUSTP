use postgres::error as postgresError;
use postgres::{Client, NoTls};
use std::env;
use std::future::Ready;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[macro_use]
extern crate serde_derive;

//Model : user struct with id,name,email
#[derive(Serialize, Deserialize)]
struct User {
    id: option<i32>,
    name: String,
    email: String,
}

//DATABASE_URL
const DB_URL: &str = !env("DATABASE_URL");

//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

//main function
fn main() {
    //set database
    if let err(e) = set_database() {
        println!("error: {}", e);
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!(0.0.0.0:8080)).unwrap();
    println!("server started at port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle.client(stream);
            }
            err(e) => {
                println!("error :{}", e);
            }
        }
    }
}

//handle_client function
fn handle_client(mut steam: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if reques_with("POST/users") => handle_post_request(r),
                r if reques_with("GET/users/") => handle_get_request(r),
                r if reques_with("GET/users") => handle_get_all_request(r),
                r if reques_with("PUT/users") => handle_put_request(r),
                r if reques_with("DELETE/users") => handle_delete_request(r),
                _ => (NOT_FOUND, "not found".to_string()),
            };

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }

        Err(e) => {
            println!("error: {}", e);
        }
    }
}

//controllers
//handle_post_request function
fn handle_post_request(request: &str) -> (String, String) {
    match (
        get_user_request_body(&request),
        Client::connect(DB_URL, NoTls),
    ) {}
}

//set database function
fn det_database() -> Result<(), postgresError> {
    //connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    //create table
    Client.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL,
    
        )",
        &[],
    )?;
}

//get_id function
fn get_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

//deserialize user from request body with the id
fn get_user_request_body(request: &str) -> Result<User, serde_json::error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
