//use std::net;
//mod common;
use request::Request;
use response::Response;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

static HTML: &'static str = include_str!("../front/index.html");

pub struct Chapel {}

impl Chapel {
    pub fn listen(&self, port: u16) {
        println!("Chapel, constructed!");

        let addr = "127.0.0.1:".to_owned() + &port.to_string();
        println!("{}", addr);

        let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {
            let _stream = stream.unwrap();
            println!("Connection established!");
            &self.on_connect(_stream);
        }
    }

    fn on_connect(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

     
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", HTML);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn handle(req: Request, res: Response, callback: &Fn()) {}
}
