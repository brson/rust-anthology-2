use std::default::Default;
use url::Url;
use serde::{Deserialize};
use anyhow::{Result, Context};

static CONFIG: &'static str =
    include_str!("../config/blog-posts.toml");

pub fn load_config() -> Result<Config> {
    toml::from_str(CONFIG)
        .context("parsing config")
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub blog_urls: Vec<Url>,
}

