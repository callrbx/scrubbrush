mod config;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "scrubbrush", about = "Video file batch conversion tool")]
struct Opt {
    #[structopt(parse(from_os_str))]
    config_path: Option<PathBuf>,
}

fn main() {
    let args = Opt::from_args();

    let config_path = match args.config_path {
        Some(conf_path) => conf_path,
        None => PathBuf::from("./sbconfig.toml"),
    };

    let config = config::Config::parse_config(config_path);

    println!("{:?}", config);
}
