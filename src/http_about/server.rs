use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use tokio::runtime::Runtime;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {

    // sleep for 5 seconds
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    Ok(Response::new(Body::from("Hello, World!")))

}

fn main() {
    // Create the runtime
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        // Define the address for the server to listen on
        let addr = ([127, 0, 0, 1], 3000).into();

        // Create a service that handles each request
        let make_svc = make_service_fn(|_conn| {
            async { Ok::<_, Infallible>(service_fn(handle_request)) }
        });

        // Create and run the server
        let server = Server::bind(&addr).serve(make_svc);

        println!("Listening on http://{}", addr);

        // Run this server for... forever!
        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }
    });
}