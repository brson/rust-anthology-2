use std::fs;
use std::default::Default;
use url::Url;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

static BLOG_FILE: &'static str = "./config/blog-posts-old.toml";

pub fn load_config_old() -> Result<ConfigOld> {
    let blogs = fs::read_to_string(BLOG_FILE)
        .context("reading blog file")?;
    toml::from_str(&blogs)
        .context("parsing config")
}

#[derive(Deserialize, Debug)]
pub struct ConfigOld {
    pub blog_urls: Vec<Url>,
}

static BLOG_FILE_2: &'static str = "./config/blog-posts-2.toml";

pub fn convert() -> Result<()> {
    let config = load_config_old()?;

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
    Introduction,
    ExperienceReports,
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
}

impl Default for Category {
    fn default() -> Category {
        Category::Uncategorized
    }
}
