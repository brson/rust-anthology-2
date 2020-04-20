use std::default::Default;
use url::Url;
use serde::{Deserialize};
use anyhow::{Result, Context};

static CONFIG: &'static str =
    include_str!("../config/blog-posts.toml");

#[derive(Deserialize, Debug)]
pub struct Config {
    pub blog_posts: Vec<BlogPost>,
}

#[derive(Deserialize, Debug)]
pub struct BlogPost {
    pub url: Url,
    pub category: Option<Category>,
}

#[derive(Deserialize, Debug)]
pub enum Category {
    Uncategorized,
}

impl Default for Category {
    fn default() -> Category {
        Category::Uncategorized
    }
}

pub fn load_config() -> Result<Config> {
    toml::from_str(CONFIG)
        .context("parsing config")
}


