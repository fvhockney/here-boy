use hyper::body::HttpBody as _;
use hyper::http::uri::Scheme;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::process::exit;
use structopt::StructOpt;
use tokio::fs::File;
use tokio::prelude::*;

mod cli;
use cli::Cli;

mod errors;
use errors::MockError;

mod config;
use config::{Config, Endpoint};

pub type LResult<T> = Result<T, MockError>;

async fn http_request(uri: &hyper::Uri) -> hyper::client::ResponseFuture {
    let client = Client::new();
    client.get(uri.clone())
}

async fn https_request(uri: &hyper::Uri) -> hyper::client::ResponseFuture {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    client.get(uri.clone())
}

async fn make_request(endpoint: Endpoint) -> LResult<()> {
    let uri = endpoint.get_uri()?;
    let req = match uri.scheme() {
        Some(x) if x == &Scheme::HTTPS => Ok(https_request(&uri).await),
        Some(x) if x == &Scheme::HTTP => Ok(http_request(&uri).await),
        Some(_) => Err(MockError::UnknownScheme(uri.to_string())),
        None => Err(MockError::NoScheme(uri.to_string())),
    };
    let mut resp = req?.await.map_err(|_| MockError::UnableToGet)?;
    if resp.status().is_success() || resp.status().is_redirection() {
        let file_name = &endpoint.file;
        let mut file = File::create(file_name)
            .await
            .map_err(|_| MockError::UnableToCreateFile(file_name.to_path_buf()))?;
        while let Some(chunk) = resp.body_mut().data().await {
            file.write_all(&chunk.map_err(|_| MockError::NoChunk)?)
                .await
                .map_err(|_| MockError::UnableToWriteToFile(file_name.to_path_buf()))?;
        }
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
