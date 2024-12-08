use hyper::{Client, Uri};
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    // Create a client
    let client = Client::new();

    // Define the URI to send the request to
    let uri: Uri = "http://127.0.0.1:3000".parse().unwrap();

    // Send a GET request
    match client.get(uri).await {
        Ok(res) => {
            println!("Response: {}", res.status());

            // Print the body
            let body_bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
            let body = String::from_utf8(body_bytes.to_vec()).unwrap();
            println!("Body: {}", body);
        }
        Err(err) => {
            eprintln!("Request error: {}", err);
        }
    }
}