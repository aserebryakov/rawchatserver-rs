use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;


fn read_line(mut stream: &TcpStream) -> Result<String, std::io::Error> {
    let mut line = String::new();
    let mut end = false;

    while !end {
        let mut b : [u8; 1] = [0];
        stream.read(&mut b)?;

        match b[0] as char {
            '\n' | '\0' => end = true,
            _ => line.push(b[0] as char),
        }
    }

    Ok(line)
}


fn handle_client(mut stream: TcpStream, server_tx : Sender<String>) -> Result<Sender<String>, std::io::Error> {
    let builder = thread::Builder::new();
    let (sender , receiver) : (Sender<String>, Receiver<String>)  = channel();

    builder.spawn(move || {
        let _ = stream.write("Greetings!\n\0".as_bytes()).unwrap();
        let _ = stream.write("Please enter your nickname: ".as_bytes()).unwrap();

        let nickname = read_line(&stream).unwrap();

        let _ = server_tx.send(nickname.clone());
        let _ = stream.write(&receiver.recv().unwrap().into_bytes()).unwrap();
    }).unwrap();

    Ok(sender)
}


fn main() {
    println!("Initializing...");

    let listener = TcpListener::bind("127.0.0.1:40000").unwrap();
    let (tx, rx) : (Sender<String>, Receiver<String>)  = channel();

    println!("Waiting for clients...");

    for stream in listener.incoming() {
        println!("Client is connected...");

        let client_tx = handle_client(stream.unwrap(), tx.clone()).unwrap();
        let nickname = rx.recv().unwrap();
        let greeting = format!("Welcome, {}!\n", nickname);

        println!("Clients nickname is {}", nickname);
        client_tx.send(greeting).unwrap();
    }
}
