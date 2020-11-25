use crate::errors::MockError;
use crate::LResult;
use hyper::Uri;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Endpoint {
    pub uri: String,
    pub file: PathBuf,
}

impl Endpoint {
    pub fn get_uri(&self) -> Result<Uri, MockError> {
        self.uri
            .parse()
            .map_err(|_| MockError::UnparsableUri(self.uri.to_string()))
    }
    fn normalize(&mut self, base_uri: &Option<String>, file_path_prefix: &Option<PathBuf>) {
        if let Some(uri) = base_uri {
            let mut u = uri.clone();
            u.push_str(&self.uri.clone());
            self.uri = u
        }
        if let Some(path_prefix) = file_path_prefix {
            let mut pp = path_prefix.clone();
            pp.push(&self.file.clone());
            self.file = pp
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub base_uri: Option<String>,
    pub file_path_prefix: Option<PathBuf>,
    pub endpoints: Vec<Endpoint>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoints: vec![],
            base_uri: None,
            file_path_prefix: None,
        }
    }
}

impl Config {
    pub fn normalize(
        &mut self,
        base_uri: Option<String>,
        file_path_prefix: Option<PathBuf>,
    ) -> LResult<()> {
        let endpoints_iter = self.endpoints.iter_mut();
        let uri = base_uri.or(self.base_uri.clone());
        let path_prefix = file_path_prefix.or(self.file_path_prefix.clone());
        if path_prefix.is_some() {
            fs::create_dir_all(path_prefix.as_ref().unwrap()).map_err(|_| {
                MockError::UnableToCreateFile(path_prefix.as_ref().unwrap().to_path_buf())
            })?;
        }
        for e in endpoints_iter {
            e.normalize(&uri, &path_prefix)
        }
        Ok(())
    }

    pub fn load(config_path: PathBuf) -> LResult<Config> {
        if !config_path.exists() {
            Err(MockError::NoConfigFound(config_path.clone()))?
        }
        let config: Config = confy::load_path(config_path.clone())
            .map_err(|_| MockError::MalformedConfig(config_path))?;
        Ok(config)
    }

    pub fn convert(&self, json_path: &PathBuf) -> LResult<()> {
        fs::create_dir_all(&json_path.parent().unwrap()).map_err(|_| {
            MockError::CantCreatePaths(
                json_path
                    .parent()
                    .unwrap_or(&PathBuf::from("").to_path_buf())
                    .to_path_buf(),
            )
        })?;
        let json_file = fs::File::create(&json_path)
            .map_err(|_| MockError::CantCreatePaths(json_path.clone()))?;
        serde_json::to_writer_pretty(json_file, &self.endpoints)
            .map_err(|_| MockError::UnableToWriteToFile(json_path.to_path_buf()))?;
        Ok(())
    }
}
