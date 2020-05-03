use std::fs;
use std::default::Default;
use url::Url;
use serde::{Deserialize};
use anyhow::{Result, Context};

static CONFIG: &'static str =
    include_str!("../config/blog-posts.toml");

static BLOG_FILE: &'static str = "./config/blog-posts.toml";

pub fn load_config() -> Result<Config> {
    let blogs = fs::read_to_string(BLOG_FILE)
        .context("reading blog file")?;
    toml::from_str(&blogs)
        .context("parsing config")
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub blog_urls: Vec<Url>,
}

