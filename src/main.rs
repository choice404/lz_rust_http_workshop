/*
 * Copyright (C) 2025 Austin Choi
 * See end of file for extended copyright information
 */

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let str = stream.unwrap();
        handle_request(str);
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut request = String::new();
    let mut reader = BufReader::new(&mut stream);

    reader.read_line(&mut request).unwrap();

    println!("Request: {}", request);

    let (status, content) = match request.as_str().trim() {
        "GET / HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            "<html><head><title>200 Ok</title></head><body><h1>Hello, World!</h1></body></html>"
        ),
        _ => (
            "HTTP/1.1 404 Not Found",
            "<html><head><title>404 Not Found</title></head><body><h1>404 Not Found</h1></body></html>"
        )
    };

    stream
        .write_all(
            format!(
                "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                status,
                content.len(),
                content,
            )
            .as_bytes(),
        )
        .unwrap();
}

/*
 * Copyright (C) 2025 Austin Choi
 *
 * lz_rust_workshop
 * This repo contains the documentation and files for the Layer Zero workshop on building your own HTTP server in Rsut.
 *
 * This code is licensed under the GNU General Public License 3.0.
 * Please see the LICENSE file in the root directory of this project for the full license details.
 */

