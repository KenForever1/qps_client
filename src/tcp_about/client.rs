
use crate::infer_client::InferClient;

use std::net::TcpStream;
use std::io::{self, Write, Read};

pub(crate) struct TcpClient{
    stream: TcpStream,
    buffer: [u8; 512],
    url: String,
}

impl TcpClient {
    pub (crate) fn new(url: String) -> TcpClient {
        let stream = TcpStream::connect(&url).unwrap();
        TcpClient{stream, buffer: [0; 512], url}
    }
    
    pub (crate) fn get_url(&self) -> &str {
        &self.url
   
     }
}

impl InferClient for TcpClient {
    fn infer(&mut self)-> io::Result<()> {
        let msg = b"Hello, server!";
        self.stream.write_all(msg)?;
    
        let bytes_read = self.stream.read(&mut self.buffer)?;
        println!("Received from server: {}", String::from_utf8_lossy(&self.buffer[..bytes_read]));    
        // let _ = self.stream.read(&mut self.buffer)?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut client = TcpClient::new("127.0.0.1:7878".to_string());
    println!("Connected to the server : {}", client.get_url());
    client.infer()?;

    Ok(())
}