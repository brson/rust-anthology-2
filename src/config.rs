use std::fmt;
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
            publish: Default::default(),
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
    pub publish: bool,
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

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Category::*;
        let lbl = match self {
            Introduction => "Intro to Rust",
            Experience => "Experience Reports",
            Ownership => "Ownership",
            Traits => "Traits",
            Language => "The Rust Language",
            Iterators => "Iterators",
            ConcurrencyAndParallelism => "Concurrency and Parallelism",
            InPractice => "Rust in Practice",
            Idioms => "Idiomatic Rust",
            Macros => "Macros",
            Unsafe => "Unsafe Rust",
            Async => "Async",
            Web => "Web Programming",
            Systems => "Systems Programming",
            Embedded => "Embedded Systems",
            Wasm => "Web Assembly",
            TypeSystems => "Fun With Type Systems",
            Internals => "Compiler Internals",
            Culture => "Rust Culture",
            Uncategorized => "Uncategorized",
        };
        write!(f, "{}", lbl)
    }
}
