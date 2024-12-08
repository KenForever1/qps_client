use std::net::{TcpListener, TcpStream};
use std::io::{self, Write, Read};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }

        // sleep for a while to simulate work
        thread::sleep(std::time::Duration::from_millis(1000));

        // Echo the received data
        stream.write_all(&buffer[..bytes_read])?;
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}