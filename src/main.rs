#![allow(unused)]

use reqwest::StatusCode;
use reqwest::blocking::Client as HttpClient;
use regex::Regex;
use std::io::Write;
use url::Url;
use serde::Deserialize;
use log::{info, debug, error};
use anyhow::{Result, Context, bail, anyhow};
use structopt::StructOpt;
use std::path::PathBuf;
use crate::http_cache::HttpCache;

mod http_cache;
mod html;

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
    Fetch(FetchCmd),
    WalkTags(WalkTagsCmd),
    ExtractArticle(ExtractArticle),
}

#[derive(StructOpt, Debug)]
struct FetchCmd {
    url_regex: String,
}

#[derive(StructOpt, Debug)]
struct WalkTagsCmd {
    url_regex: String,
}

#[derive(StructOpt, Debug)]
struct ExtractArticle {
    url_regex: String,
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

struct CmdOpts<T> {
    global_opts: GlobalOpts,
    config: Config,
    cmd: T,
}

fn main() -> Result<()> {
    let env = env_logger::Env::new().default_filter_or("info");
    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .target(env_logger::Target::Stdout)
        .init();

    let opts = Opts::from_args();

    debug!("opts: {:#?}", opts);

    let global_opts = opts.global_opts;
    let config = load_config(CONFIG)?;

    match opts.command {
        Command::DumpConfig => {
            info!("config: {:#?}", config);
            Ok(())
        }
        Command::Fetch(cmd) => {
            run_fetch(CmdOpts { global_opts, config, cmd })
        }
        Command::WalkTags(cmd) => {
            run_walk_tags(CmdOpts { global_opts, config, cmd })
        }
        Command::ExtractArticle(cmd) => {
            run_extract_article(CmdOpts { global_opts, config, cmd })
        }
    }
}

fn load_config(s: &str) -> Result<Config> {
    toml::from_str(s)
        .context("parsing config")
}

fn run_fetch(cmd: CmdOpts<FetchCmd>) -> Result<()> {
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|post| {
        info!("{}", post);
        Ok(())
    })
}

type PostHandler = dyn Fn(String) -> Result<()>;

fn for_each_post(opts: &GlobalOpts, config: &Config, url_regex: &str, f: &PostHandler) -> Result<()> {
    let regex = Regex::new(url_regex)
        .context("building regex")?;
    let cache_dir = opts.data_dir.join("http-cache");
    let mut client = HttpCache::new(cache_dir);

    for post in &config.blog_posts {
        if regex.is_match(&post.url.as_str()) {
            info!("fetching {}", post.url);
            let page = client.get(&post.url)?;
            f(page)?;
        }
    }
    
    Ok(())
}

fn run_walk_tags(cmd: CmdOpts<WalkTagsCmd>) -> Result<()> {
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|post| {
        html::walk_tags(&post)?;
        Ok(())
    })
}

fn run_extract_article(cmd: CmdOpts<ExtractArticle>) -> Result<()> {
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|post| {
        if let Err(e) = html::extract_article(&post) {
            error!("{}", e);
        }
        Ok(())
    })
}
