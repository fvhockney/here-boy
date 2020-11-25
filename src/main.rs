use hyper::body::HttpBody as _;
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

async fn make_request(endpoint: Endpoint) -> LResult<()> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = endpoint.get_uri()?;
    let mut resp = client
        .get(uri.clone())
        .await
        .map_err(|_| MockError::UnableToGet)?;
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