use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

use config::ProxyConfig;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use lazy_static::lazy_static;
use tokio::net::TcpListener;

mod config;
use config::parser::load_config;

// global config variable, essentially
lazy_static! {
    static ref CONFIG: Mutex<Option<Arc<ProxyConfig>>> = Mutex::new(None);
}

// derived heavily from the Hyper example
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // because of the "await," this function will wait for the config to load before continuing to bind the listener
    // tldr: sets the global CONFIG variable to the loaded ProxyConfig object, 
    // making it accessible to the rest of the application in a thread-safe manner.
    let config = Arc::new(load_config("src/test_config.yaml").await?);
    {
        let mut config_ref = CONFIG.lock().unwrap();
        *config_ref = Some(config.clone());
    }

    // gotta print the config at start up, right?
    println!("Config: {:?}", config);

    // create a TcpListener and bind it to addr
    let addr = SocketAddr::from(([127, 0, 0, 1], config.proxy_port));
    let listener = TcpListener::bind(addr).await?;

    // loop continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
    
        let config_clone = config.clone(); // clone the Arc to move into the tokio task
        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            let hs = HandlerService { config: config_clone }; // Create a new HandlerService here
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(|req| hs.call(req)))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[derive(Clone)]
struct HandlerService {
    config: Arc<ProxyConfig>,  // Use Arc to make proxyConfig shareable among tasks
}

impl HandlerService {
    async fn call(&self, req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
        let uri = req.uri().path();

        if let Some(route) = self.config.routes.get(uri) {
            // a simluation of a proxy request
            let resp_body = format!("Routing to {}:{}", route.host, route.port);
            Ok(Response::new(Full::new(Bytes::from(resp_body))))
        } else {
            Ok(Response::new(Full::new(Bytes::from("Route not found"))))
        }
    }
}
