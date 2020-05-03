use std::fs;
use std::default::Default;
use url::Url;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

static BLOG_FILE_OLD: &'static str = "./config/blog-posts-old.toml";

pub fn load_config_old() -> Result<ConfigOld> {
    let blogs = fs::read_to_string(BLOG_FILE_OLD)
        .context("reading blog file")?;
    toml::from_str(&blogs)
        .context("parsing config")
}

#[derive(Deserialize, Debug)]
pub struct ConfigOld {
    pub blog_urls: Vec<Url>,
}

static BLOG_FILE: &'static str = "./config/blog-posts.toml";

pub fn load_config() -> Result<Config> {
    let blogs = fs::read_to_string(BLOG_FILE)
        .context("reading blog file")?;
    toml::from_str(&blogs)
        .context("parsing config")
}

pub fn convert() -> Result<()> {
    let config = load_config_old()?;

    let posts = config.blog_urls.into_iter().map(|url| {
        BlogPost {
            url,
            category: Default::default(),
            broken: Default::default(),
        }
    });

    let config2 = Config { blog_posts: posts.collect() };

    let toml = toml::to_string(&config2)?;

    fs::write(BLOG_FILE, &toml)
        .context("writing blog-posts-2")?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub blog_posts: Vec<BlogPost>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogPost {
    pub url: Url,
    #[serde(default)]
    pub category: Category,
    #[serde(default)]
    pub broken: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Introduction,
    Experience,
    Ownership,
    Traits,
    Language,
    Iterators,
    ConcurrencyAndParallelism,
    InPractice,
    Idioms,
    Macros,
    Unsafe,
    Async,
    Web,
    Systems,
    Embedded,
    Wasm,
    TypeSystems,
    Internals,
    Culture,
    Uncategorized,
}

impl Default for Category {
    fn default() -> Category {
        Category::Uncategorized
    }
}
