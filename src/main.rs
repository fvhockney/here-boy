use hyper::body::HttpBody as _;
use hyper::client::ResponseFuture;
use hyper::http::uri::Scheme;
use hyper::Client;
use hyper::Response;
use hyper::StatusCode;
use hyper::Uri;
use hyper_tls::HttpsConnector;
use std::process::exit;
use structopt::StructOpt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
//use tokio::prelude::*;

mod cli;
use cli::Cli;

mod errors;
use errors::MockError;

mod config;
use config::{Config, Endpoint};

pub type LResult<T> = Result<T, MockError>;

async fn http_request(uri: &Uri) -> ResponseFuture {
    let client = Client::new();
    client.get(uri.clone())
}

async fn https_request(uri: &Uri) -> ResponseFuture {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    client.get(uri.clone())
}

async fn build_request(uri: &Uri) -> LResult<ResponseFuture> {
    match uri.scheme() {
        Some(x) if x == &Scheme::HTTPS => Ok(https_request(&uri).await),
        Some(x) if x == &Scheme::HTTP => Ok(http_request(&uri).await),
        Some(_) => Err(MockError::UnknownScheme(uri.to_string())),
        None => Err(MockError::NoScheme(uri.to_string())),
    }
}

fn is_successful(status: &StatusCode) -> bool {
    status.is_success() || status.is_redirection()
}

async fn write_resp_to_file(
    response: &mut Response<hyper::Body>,
    endpoint: &Endpoint,
) -> LResult<()> {
    let file_name = &endpoint.file;
    let mut file = File::create(file_name)
        .await
        .map_err(|_| MockError::UnableToCreateFile(file_name.to_path_buf()))?;
    while let Some(chunk) = response.body_mut().data().await {
        file.write_all(&chunk.map_err(|_| MockError::NoChunk)?)
            .await
            .map_err(|_| MockError::UnableToWriteToFile(file_name.to_path_buf()))?;
    }
    Ok(())
}

async fn make_request(endpoint: Endpoint) -> LResult<()> {
    let uri = endpoint.get_uri()?;
    let req = build_request(&uri).await?;
    let mut resp = req.await.map_err(|_| MockError::UnableToGet)?;
    if is_successful(&resp.status()) {
        write_resp_to_file(&mut resp, &endpoint).await?;
        Ok(())
    } else {
        Err(MockError::RequestFailed(
            uri.to_string(),
            resp.status().to_string(),
        ))
    }
}

async fn run() -> LResult<()> {
    let args = Cli::from_args();

    let mut config = Config::load(args.config)?;
    config.normalize(args.base_uri, args.file_path_prefix)?;

    if args.convert_config == true {
        let json_path = args.converted_config_path;
        config.convert(&json_path)?;
    }

    let mut h = vec![];
    for endpoint in config.endpoints.into_iter() {
        h.push(tokio::task::spawn(
            async move { make_request(endpoint).await },
        ))
    }
    for handle in h {
        match handle.await {
            Ok(x) => match x {
                Ok(_) => {}
                Err(x) => eprintln!("{}", x),
            },
            Err(x) => eprintln!("err {}", x),
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn main() {
    let res = run().await;
    if let Err(err) = res {
        eprintln!("{}", err);
        exit(1);
    }
}
