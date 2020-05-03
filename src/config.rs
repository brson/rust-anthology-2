use std::fs;
use std::default::Default;
use url::Url;
use serde::{Serialize, Deserialize};
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

static BLOG_FILE_2: &'static str = "./config/blog-posts-2.toml";

pub fn convert() -> Result<()> {
    let config = load_config()?;

    let posts = config.blog_urls.into_iter().map(|url| {
        BlogPost {
            url,
            category: Default::default(),
            broken: Default::default(),
        }
    });

    let config2 = Config2 { blog_posts: posts.collect() };

    let toml = toml::to_string(&config2)?;

    fs::write(BLOG_FILE_2, &toml)
        .context("writing blog-posts-2")?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config2 {
    blog_posts: Vec<BlogPost>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPost {
    url: Url,
    #[serde(default)]
    category: Category,
    #[serde(default)]
    broken: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    Uncategorized,
}

impl Default for Category {
    fn default() -> Category {
        Category::Uncategorized
    }
}
