use std::net::TcpListener;
use std::thread;
use std::net::Shutdown;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

    println!("Listening for HTTP requests on {}", listener.local_addr().unwrap());

    for stream_result in listener.incoming() {
        thread::spawn(move || {
            let ref mut stream = stream_result.unwrap();

            write!(
                stream,
                "HTTP/1.1 200 OK\r\n\
                Content-Type: text/html; charset=utf-8\r\n\
                Transfer-Encoding: chunked\r\n\
                Connection: close\r\n\
                \r\n"
            ).unwrap();

            for i in "hello_world".chars() {
                let chunk_str = format!("{}", i);
                let chunk_body: &[u8] = chunk_str.as_bytes();
                write!(stream, "{:x}\r\n", chunk_body.len())
                    .and_then(|_| stream.write_all(chunk_body))
                    .and_then(|_| write!(stream, "\r\n"))
                    .unwrap();

                thread::sleep_ms(500);
            }

            write!(stream, "0\r\n\r\n").unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
        });
    }
}
