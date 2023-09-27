use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

mod config;
use config::parser::load_config;

// derived heavily from the Hyper example
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // Load the config, just print it. that's enough for tonight
    // because of the "await," this function will wait for the config to load before continuing to bind the listener
    let config = load_config("src/test_config.yaml").await?;
    println!("Config: {:?}", config);

    // create a TcpListener and bind it to addr
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = TcpListener::bind(addr).await?;

    // loop continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // TODO: I am not sure what this means yet.
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // bind the incoming connection to our `hello` service, this is just to get us started
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Fletch, or Fern?!"))))
}
