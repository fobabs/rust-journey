use hello::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

/// Sets up a TCP listener on localhost at port 7878 and handles each incoming connection using the `handle_connection` function.
///
/// # Limitations
///
/// This server will only serve two requests before shutting down.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // The closure passed to `execute` will be executed in a separate thread.
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

/// Handles a single incoming connection.
///
/// The function reads a single line from the incoming stream and uses it to determine which file to serve.
/// The following paths are supported:
///
/// - `/` - serves hello.html
/// - `/sleep` - sleeps for 5 seconds and then serves hello.html
/// - Any other path - serves 404.html
///
/// The function reads the requested file and writes it to the stream, prepending a status line and content length header.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
