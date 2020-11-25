# Api File Mocker

Download data from API endpoints to individual files to use later. I use this for testing my api liberary integrations against actual data from the api in my test mocks.

Written with [tokio](docs.rs/tokio/0.2.23) and [hyper](docs.rs/hyper) for blazing fast concurrency.

## Features

- Configure with a toml file or via cli options.
  - Cli options will override configuration in the toml.
- Convert toml enpoints entries into json for easy consumption with javaScript mocks
- Concurrency provided by `hyper` and `tokio`
- Receive feedback when an enpoint is unreachable/unusable


## Installation

1. Clone the repo
2. `cargo build --release`
3. Make sure the binary is in your path
4. Create your config (must be done manually)
5. Run and enjoy!

## Config Reference
```toml
# ./api.toml

base_uri = "https://your.base.uri"

file_path_prefix = "./path/to/save/data/"

[[endpoints]]
uri = "/endpoint/1"
file = "endpoint_1.json"

[[endpoints]]
uri = "/endpoint/2/specific/entry"
file = "endpoint_2_entry.json"
```

## Defaults

Default config location: './api.toml'
Default json converstion of config enpoints: './api.json'

## Cli Reference
```bash
api-file-mocker 0.1.0

USAGE:
    api-file-mocker [FLAGS] [OPTIONS]

FLAGS:
        --convert-config
    -h, --help              Prints help information
    -V, --version           Prints version information
    -v

OPTIONS:
        --base-url <base-uri>
    -c, --config <config>                                   [default: api.toml]
        --converted-config-path <converted-config-path>     [default: api.json]
        --dir <file-path-prefix>
```

## NB

Paths are dumb. The program makes no attempt to merge your `base-uri` or `file-path-prefix` intelligently. If you leave off or add a `/`, that's on you. (for now)

This program is under development, expect changes.

No tests yet, so probably more than a few rough edges.
