use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::OnceLock;

use std::str::FromStr;
use sui_types::base_types::SuiAddress;

use clap::Parser;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1::Builder;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use regex::Regex;
use tokio::net::TcpListener;

static COIN_MAP: OnceLock<HashMap<String, SuiAddress>> = OnceLock::new();
static COIN_HOLDER_RE: OnceLock<Regex> = OnceLock::new();

#[derive(clap::Parser)]
#[command(author, version, about)]
pub struct Command {
    /// Run service in port
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,

    /// Pass coin package id mapping
    /// ex: usdc=0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf
    #[arg(short, long, value_parser = parse_key_val::<String, String>)]
    pub coins: Vec<(String, String)>,
}

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid COIN=0xAddress: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

async fn handler(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    if let Some(caps) = COIN_HOLDER_RE
        .get()
        .expect("COIN_HOLDER_RE should init")
        .captures(&req.uri().to_string())
    {
        let coin_map = COIN_MAP.get().expect("COIN_MAP should init");
        let package_id = coin_map[&caps["coin_type"]];
        let address = SuiAddress::from_str(&caps["addr"]).expect("address incorrect");
        let value: u64 = caps["value"].parse().expect("value incorrect");

        println!("{package_id:?}");
        println!("{value:?}");
        println!("{address:?}");
        Ok(Response::new(Full::new(Bytes::from("1"))))
    } else {
        Ok(Response::new(Full::new(Bytes::from("0"))))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Command::parse();
    let coins: HashMap<String, SuiAddress> = args
        .coins
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                SuiAddress::from_str(&v).expect("Coin package id incorrect"),
            )
        })
        .collect();

    COIN_MAP.get_or_init(|| coins);
    COIN_HOLDER_RE.get_or_init(|| {
        Regex::new(r"^/(?P<coin_type>[^/]+)/(?P<value>\d+)\?address=(?<addr>.+)$").unwrap()
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
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
