#![allow(unused)]

use std::collections::HashMap;
use std::cell::RefCell;
use std::fs;
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
use crate::config::{load_config, Config, BlogPost};
use crate::index::IndexEntry;

mod http_cache;
mod html;
mod doc;
mod convert;
mod sanitize;
mod render;
mod assets;
mod config;
mod extract;
mod index;

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
    ConvertArticle(ConvertArticle),
    RenderArticle(RenderArticle),
    CopyAssets(CopyAssets),
    ExtractTitle(ExtractTitle),
    GenerateSlug(GenerateSlug),
    WriteIndex(WriteIndex),
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
struct ConvertArticle {
    url_regex: String,
}

#[derive(StructOpt, Debug)]
struct RenderArticle {
    url_regex: String,
    #[structopt(long)]
    to_file: bool,
}

#[derive(StructOpt, Debug)]
struct CopyAssets { }

#[derive(StructOpt, Debug)]
struct ExtractTitle {
    url_regex: String,
}

#[derive(StructOpt, Debug)]
struct GenerateSlug {
    url_regex: String,
}

#[derive(StructOpt, Debug)]
struct WriteIndex {
    url_regex: String,
}

#[derive(StructOpt, Debug)]
struct GlobalOpts {
    #[structopt(default_value = "./data")]
    data_dir: PathBuf,
    /// Process posts that aren't marked for publication
    #[structopt(long)]
    unpublished: bool,
}

static RENDER_DIR: &'static str = "render";
static POST_DIR: &'static str = "p";

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
    let config = load_config()?;

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
        Command::ConvertArticle(cmd) => {
            run_convert_article(CmdOpts { global_opts, config, cmd })
        }
        Command::RenderArticle(cmd) => {
            run_render_article(CmdOpts { global_opts, config, cmd })
        }
        Command::CopyAssets(cmd) => {
            run_copy_assets(CmdOpts { global_opts, config, cmd })
        }
        Command::ExtractTitle(cmd) => {
            run_extract_title(CmdOpts { global_opts, config, cmd })
        }
        Command::GenerateSlug(cmd) => {
            run_generate_slug(CmdOpts { global_opts, config, cmd })
        }
        Command::WriteIndex(cmd) => {
            run_write_index(CmdOpts { global_opts, config, cmd })
        }
    }
}

fn run_fetch(cmd: CmdOpts<FetchCmd>) -> Result<()> {
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|_, post| {
        debug!("{}", post);
        Ok(())
    })
}

type PostHandler<'a> = dyn Fn(&BlogPost, String) -> Result<()> + 'a;

fn for_each_post(opts: &GlobalOpts, config: &Config, url_regex: &str, f: &PostHandler) -> Result<()> {
    let regex = Regex::new(url_regex)
        .context("building regex")?;
    let cache_dir = opts.data_dir.join("http-cache");
    let mut client = HttpCache::new(cache_dir);

    for post in &config.blog_posts {
        let publish = post.publish || opts.unpublished;
        if !publish {
            debug!("skipping {}", post.url);
            continue;
        }
        if regex.is_match(&post.url.as_str()) {
            info!("fetching {}", post.url);
            let page = client.get(&post.url);
            match page {
                Ok(page) => {
                    f(post, page)?;
                }
                Err(e) => {
                    error!("error: {}", e);
                }
            }
        }
    }
    
    Ok(())
}

fn run_walk_tags(cmd: CmdOpts<WalkTagsCmd>) -> Result<()> {
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|_, post| {
        html::walk_tags(&post)?;
        Ok(())
    })
}

fn run_extract_article(cmd: CmdOpts<ExtractArticle>) -> Result<()> {
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|_, post| {
        match html::extract_article_string(&post) {
            Ok(s) => {
                info!("{}", s);
            }
            Err(e) => {
                error!("{}", e);
            }
        }
        Ok(())
    })
}

fn run_convert_article(cmd: CmdOpts<ConvertArticle>) -> Result<()> {
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|meta, post| {
        match html::extract_article(&post) {
            Ok(dom) => {
                let doc = convert::from_dom(&meta, &dom);
                info!("{:#?}", doc);
            }
            Err(e) => {
                error!("{}", e);
            }
        }
        Ok(())
    })
}

fn run_render_article(cmd: CmdOpts<RenderArticle>) -> Result<()> {
    let assets = assets::AssetDirs {
        css_dir: PathBuf::from("../css/"),
    };
    
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|meta, post| {
        match html::extract_article(&post) {
            Ok(dom) => {
                let doc = convert::from_dom(&meta, &dom);
                let doc = sanitize::sanitize(doc);
                let title = extract::title(&doc);
                let doc = render::to_string(&assets, &doc)?;
                if !cmd.cmd.to_file {
                    info!("{}", doc);
                } else {
                    match title {
                        Some(title) => {
                            let file_name = sanitize::title_to_slug(title);
                            let render_dir = cmd.global_opts.data_dir.join(RENDER_DIR);
                            let post_dir = render_dir.join(POST_DIR);
                            let render_file = post_dir.join(format!("{}.html", file_name));
                            fs::create_dir_all(&post_dir)
                                .context("creating post dir")?;
                            fs::write(&render_file, doc)
                                .context("writing rendered doc")?;
                            info!("rendered at {}", render_file.display());
                        }
                        None => {
                            error!("unable to extract title");
                        }
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }
        Ok(())
    })
}

fn run_copy_assets(cmd: CmdOpts<CopyAssets>) -> Result<()> {
    let render_dir = cmd.global_opts.data_dir.join(RENDER_DIR);
    let css_dir = render_dir.join("css");
    let dirs = assets::AssetDirs {
        css_dir
    };

    assets::copy(&dirs)
}

fn run_extract_title(cmd: CmdOpts<ExtractTitle>) -> Result<()> {
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|meta, post| {
        match html::extract_article(&post) {
            Ok(dom) => {
                let doc = convert::from_dom(&meta, &dom);
                let doc = sanitize::sanitize(doc);
                let title = extract::title(&doc);
                match title {
                    Some(title) => {
                        info!("title: {}", title);
                    },
                    None => {
                        error!("no title found");
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }
        Ok(())
    })
}

fn run_generate_slug(cmd: CmdOpts<GenerateSlug>) -> Result<()> {
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|meta, post| {
        match html::extract_article(&post) {
            Ok(dom) => {
                let doc = convert::from_dom(&meta, &dom);
                let doc = sanitize::sanitize(doc);
                let title = extract::title(&doc);
                match title {
                    Some(title) => {
                        let file_name = sanitize::title_to_slug(title);
                        info!("slug: {}", file_name);
                    },
                    None => {
                        error!("no title found");
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }
        Ok(())
    })
}

fn run_write_index(cmd: CmdOpts<WriteIndex>) -> Result<()> {
    let assets = assets::AssetDirs {
        css_dir: PathBuf::from("./css/"),
    };
    let data = RefCell::new(Vec::new());
    for_each_post(&cmd.global_opts, &cmd.config, &cmd.cmd.url_regex, &|meta, post| {
        match html::extract_article(&post) {
            Ok(dom) => {
                let doc = convert::from_dom(&meta, &dom);
                let doc = sanitize::sanitize(doc);
                let title = extract::title(&doc);
                match title {
                    Some(title) => {
                        let file_name = sanitize::title_to_slug(title.clone());
                        let index_entry = IndexEntry {
                            post_meta: meta.clone(),
                            title,
                            file_name,
                        };
                        data.borrow_mut().push(index_entry);
                    },
                    None => {
                        error!("no title found");
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }
        Ok(())
    });

    let render_dir = cmd.global_opts.data_dir.join(RENDER_DIR);
    index::write(&render_dir, &assets, data.into_inner())?;
    Ok(())
}
