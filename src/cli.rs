use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab")]
pub struct Cli {
    #[structopt(long)]
    pub convert_config: bool,
    #[structopt(long, default_value = "api.json")]
    pub converted_config_path: PathBuf,
    #[structopt(long = "dir")]
    pub file_path_prefix: Option<PathBuf>,
    #[structopt(long = "base-url")]
    pub base_uri: Option<String>,
    #[structopt(short, long, default_value = "api.toml")]
    pub config: PathBuf,
    #[structopt(short, parse(from_occurrences))]
    pub verbose: u8,
}
