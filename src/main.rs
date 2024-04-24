use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let mut port = 7878;

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        for i in args[1..].iter() {
            if let Some(e) = i.strip_prefix("-p=") {
                if let Ok(p) = e.parse() {
                    port = p;
                } else {
                    eprintln!("Failed to parse port as an integer.");
                    std::process::exit(1);
                }
            }
        }
    }

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    println!("Opened server on port {port}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        thread::spawn(|| {
            handle_connection(stream);
        });
    }

    /*for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }*/
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let r: Vec<&str> = request_line.split_whitespace().collect();
    let reqtype = r[2];
    let status_line = "HTTP/1.1 200 OK";
    let mut page = String::new();

    if r[0].to_lowercase() == "get" {
        match r[1].trim() {
            "/" => page = fs::read_to_string("index.html").unwrap(),
            "/test" => page = fs::read_to_string("test.html").unwrap(),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                page = fs::read_to_string("test.html").unwrap();
            }
            _ => {
                let status_line = format!("{reqtype} 404 NOT FOUND");
                let contents = fs::read_to_string("404.html").unwrap();
                let length = contents.len();

                let response =
                    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                stream.write_all(response.as_bytes()).unwrap();
            }
        }
        let response = format!(
            "{status_line}\r\nContent-Length: {}\r\n\r\n{page}",
            page.len()
        );
        stream.write_all(response.as_bytes()).unwrap();
    } else if r[0] == "POST" {
    } else {
        panic!("Malformed HTTP request");
    }
}
