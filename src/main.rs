use std::net::SocketAddr;
use std::convert::Infallible;

use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::server::conn::http1::Builder;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use http_body_util::Full;
use tokio::net::TcpListener;

async fn handler(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("1"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = Builder::new()
                .serve_connection(io, service_fn(handler))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
