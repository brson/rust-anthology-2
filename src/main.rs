#![allow(unused)]

use serde::Deserialize;
use log::{info, debug};
use anyhow::Result;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(flatten)]
    gopts: GlobalOpts,
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

#[derive(Deserialize)]
struct Config {
}

static CONFIG: &'static str = include_str!("config.toml");

fn main() -> Result<()> {
    env_logger::init();

    let opts = Opts::from_args();

    debug!("opts: {:#?}", opts);

    let config = load_config(CONFIG)?;

    Ok(())
}

fn load_config(s: &str) -> Result<Config> {
    panic!()
}
