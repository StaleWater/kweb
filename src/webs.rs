use std::{
    fs,
    net::{TcpStream, TcpListener},
    io::{BufReader, prelude::*},
};
use super::threading::ThreadPool;

pub struct WebServer<'a> {
    pool: ThreadPool,
    bind_address: &'a str,
}

impl<'a> WebServer<'a> {
    pub fn new(bind_address: &str, num_threads: usize) -> WebServer {
        WebServer {
            pool: ThreadPool::new(num_threads),
            bind_address,
        }
    }

    pub fn start(&self) -> std::io::Result<()> {
        let tcp = TcpListener::bind(self.bind_address)?;

        for stream in tcp.incoming() {
            let stream = stream.expect("failed to unwrap the stream");
            self.pool.execute(|| handle_stream(stream));
        }

        Ok(())
    }

    
}

fn handle_stream(mut stream: TcpStream) {
    let buffer = BufReader::new(&mut stream);
    let request_top = buffer.lines().next().unwrap().unwrap();
    let mut request_top = request_top.split_whitespace();

    if request_top.next().unwrap() != "GET" {return}

    let filename = request_top.next().unwrap();
    let filename = route(filename);

    let mut status_line = "HTTP/1.1 200 FKM";

    let content = fs::read_to_string(&filename);
    let content = match content {
        Ok(x) => x,
        Err(_) => {
            println!("{filename} does not exist");
            status_line = "HTTP/1.1 404 NOTFOUND";
            "".to_string()
        }
    };

    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn route(path: &str) -> String {

    let mut path = path.to_string();

    if path == "/" {
        path = "/index".to_string();
    }

    if !path.contains(".") {
        path = format!("./www/html{path}.html");
    }
    else {
        let ext = path.split(".").last().unwrap();
        let folder: &str; 
        match ext {
            "css" => folder = "./www/css",
            "html" => folder = "./www/html",
            "js" => folder = "./www/js",
            _ => folder = "./www/assets",
        }
        path = format!("{folder}{path}");
    }

    path
}
