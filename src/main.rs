use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::OnceLock;

use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{StructTag, TypeTag};
use sui_json_rpc_types::SuiMoveStruct;
use sui_json_rpc_types::SuiMoveValue;
use sui_json_rpc_types::SuiObjectResponseQuery;
use sui_json_rpc_types::SuiParsedData;
use sui_json_rpc_types::SuiParsedMoveObject;
use sui_json_rpc_types::{SuiObjectDataFilter, SuiObjectDataOptions};
use sui_sdk::SuiClientBuilder;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::Identifier;

use clap::Parser;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_MAX_AGE,
};
use hyper::server::conn::http1::Builder;
use hyper::service::service_fn;
use hyper::Method;
use hyper::StatusCode;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use regex::Regex;
use tokio::net::TcpListener;

static COIN_MAP: OnceLock<HashMap<String, ObjectID>> = OnceLock::new();
static COIN_HOLDER_RE: OnceLock<Regex> = OnceLock::new();
static COIN_ADDR: OnceLock<SuiAddress> = OnceLock::new();

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

fn add_headers(mut response: Response<Full<Bytes>>) -> Response<Full<Bytes>> {
    response.headers_mut().insert(
        ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Content-Type"),
    );
    response.headers_mut().insert(
        ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, OPTIONS"),
    );
    response.headers_mut().insert(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("https://dashboard.galxe.com"),
    );
    response
        .headers_mut()
        .insert(ACCESS_CONTROL_MAX_AGE, HeaderValue::from_static("604800"));
    response
}

async fn handler(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    if req.method() == Method::OPTIONS {
        let mut res = add_headers(Response::new(Full::new(Bytes::from(""))));
        *res.status_mut() = StatusCode::NO_CONTENT;
        return Ok(res);
    }

    if let Some(caps) = COIN_HOLDER_RE
        .get()
        .expect("COIN_HOLDER_RE should init")
        .captures(&req.uri().to_string())
    {
        let coin_map = COIN_MAP.get().expect("COIN_MAP should init");
        let package = coin_map[&caps["coin_type"]];
        let address = SuiAddress::from_str(&caps["addr"]).expect("address incorrect");
        let expect_value: u64 = caps["value"].parse().expect("value incorrect");

        let sui = if let Ok(sui) = SuiClientBuilder::default().build_mainnet().await {
            sui
        } else {
            return Ok(add_headers(Response::new(Full::new(Bytes::from("-1")))));
        };
        if let Ok(coins) = sui
            .read_api()
            .get_owned_objects(
                address,
                Some(SuiObjectResponseQuery {
                    filter: Some(SuiObjectDataFilter::StructType(StructTag {
                        address: AccountAddress::TWO,
                        module: Identifier::new("coin").unwrap(),
                        name: Identifier::new("Coin").unwrap(),
                        type_params: vec![TypeTag::Struct(Box::new(StructTag {
                            address: AccountAddress::from_bytes(package.into_bytes()).unwrap(),
                            module: Identifier::new(&caps["coin_type"]).unwrap(),
                            name: Identifier::new(caps["coin_type"].to_uppercase()).unwrap(),
                            type_params: Vec::new(),
                        }))],
                    })),
                    options: Some(SuiObjectDataOptions {
                        show_content: true,
                        ..Default::default()
                    }),
                }),
                None,
                None,
            )
            .await
        {
            let mut value = 0u64;
            for coin in coins.data.iter() {
                if let Some(SuiParsedData::MoveObject(SuiParsedMoveObject {
                    fields: SuiMoveStruct::WithFields(fields),
                    ..
                })) = &coin.data.as_ref().expect("coin has data").content
                {
                    match fields.get("balance") {
                        Some(SuiMoveValue::Number(v)) => {
                            value += *v as u64;
                        }
                        Some(SuiMoveValue::String(s)) => {
                            let v: u64 = s.parse().expect("balance should be numbers");
                            value += v;
                        }
                        _ => (),
                    }
                }
                if value >= expect_value {
                    // Already more than expected value
                    break;
                }
            }
            if value >= expect_value {
                return Ok(add_headers(Response::new(Full::new(Bytes::from("1")))));
            }
        }
    }
    Ok(add_headers(Response::new(Full::new(Bytes::from("0")))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Command::parse();
    let coins: HashMap<String, ObjectID> = args
        .coins
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                ObjectID::from_str(&v).expect("Coin package id incorrect"),
            )
        })
        .collect();

    COIN_MAP.get_or_init(|| coins);
    COIN_HOLDER_RE.get_or_init(|| {
        Regex::new(r"^/(?P<coin_type>[^/]+)/(?P<value>\d+)\?address=(?<addr>.+)$").unwrap()
    });
    COIN_ADDR.get_or_init(|| {
        SuiAddress::from_str("0x0000000000000000000000000000000000000000000000000000000000000002")
            .unwrap()
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
