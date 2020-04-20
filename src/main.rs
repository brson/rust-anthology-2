#![allow(unused)]

use std::io::Write;
use url::Url;
use serde::Deserialize;
use log::{info, debug, error};
use anyhow::{Result, Context};
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(flatten)]
    global_opts: GlobalOpts,
}

#[derive(StructOpt, Debug)]
enum Command {
    DumpConfig,
}

#[derive(StructOpt, Debug)]
struct GlobalOpts {
    #[structopt(default_value = "./data")]
    data_dir: PathBuf,
}

static CONFIG: &'static str = include_str!("config.toml");

#[derive(Deserialize, Debug)]
struct Config {
    blog_posts: Vec<BlogPost>,
}

#[derive(Deserialize, Debug)]
struct BlogPost {
    url: Url,
}

fn main() -> Result<()> {
    let env = env_logger::Env::new().default_filter_or("info");
    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .init();

    let opts = Opts::from_args();

    debug!("opts: {:#?}", opts);

    let config = load_config(CONFIG)?;

    match opts.command {
        Command::DumpConfig => {
            info!("config: {:#?}", config);
        }
    }

    Ok(())
}

fn load_config(s: &str) -> Result<Config> {
    toml::from_str(s)
        .context("parsing config")
}
