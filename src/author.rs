use std::io::Write;
use log::info;
use std::fs::{self, File};
use crate::render;
use crate::sanitize;
use std::path::Path;
use crate::assets::AssetDirs;
use crate::assets::{RESET_CSS_FILE, MAIN_CSS_FILE, BLOG_CSS_FILE};
use std::iter::FromIterator;
use anyhow::{Result, Context};
use std::collections::{BTreeMap, BTreeSet};
use std::collections::btree_map::Entry;
use url::Url;
use crate::config::{Config, Author};
use crate::index::IndexEntry;

pub fn create_author_maps(config: &Config) -> Result<AuthorMaps> {
    let mut blog_post_author = BTreeMap::new();
    let mut author_blog_posts = BTreeMap::new();

    for post in &config.blog_posts {
        for author in &config.authors {
            if let Some(blog_url) = &author.blog {
                if post.url.as_str().starts_with(blog_url.as_str()) {
                    blog_post_author.insert(post.url.clone(), author.name.clone());

                    match author_blog_posts.entry(author.name.clone()) {
                        Entry::Vacant(v) => {
                            v.insert(BTreeSet::from_iter(Some(post.url.clone())));
                        }
                        Entry::Occupied(mut v) => {
                            v.get_mut().insert(post.url.clone());
                        }
                    }
                }
            }
        }
    }

    Ok(AuthorMaps {
        blog_post_author, author_blog_posts
    })
}

pub type AuthorName = String;

#[derive(Debug)]
pub struct AuthorMaps {
    pub blog_post_author: BTreeMap<Url, AuthorName>,
    pub author_blog_posts: BTreeMap<AuthorName, BTreeSet<Url>>,
}

pub fn write_pages(dir: &Path, authors: &Vec<Author>, assets: &AssetDirs, index_data: Vec<IndexEntry>, author_maps: AuthorMaps) -> Result<()> {
    for author in authors {
        let has_pages = author_maps.author_blog_posts.contains_key(&author.name);
        write_author_page(dir, author, assets, &index_data, &author_maps)?;
    }

    Ok(())
}

fn write_author_page(dir: &Path, author: &Author, assets: &AssetDirs, index_data: &Vec<IndexEntry>, author_maps: &AuthorMaps) -> Result<()> {
    let dir = dir.join("a");
    fs::create_dir_all(&dir)?;
    let author_slug = sanitize::name_to_slug(author.name.clone());
    let author_file = dir.join(format!("{}.html", author_slug));
    let mut file = File::create(&author_file)
        .context("opening author file")?;

    writeln!(file, "<!doctype html>");
    writeln!(file, "<html lang='en'>");

    let header_meta = render::HeaderMeta {
        title: Some("The Rust Docuniverse".to_string()),
    };
    
    render::render_head(&mut file, assets, &header_meta);
    render_body(&mut file, author, index_data, author_maps)?;

    writeln!(file, "</html>");

    info!("author written to {}", author_file.display());

    Ok(())
}

fn render_body(file: &mut File, author: &Author, index_data: &Vec<IndexEntry>, author_maps: &AuthorMaps) -> Result<()> {
    writeln!(file, "<body>");
    writeln!(file, "<main>");

    writeln!(file, "<h1>{}</h1>", author.name);

    if let Some(github) = &author.github {
        writeln!(file, "<div>");
        writeln!(file, "<p>GitHub: <a href='https://github.com/{}'>@{}</a></p>", github, github);
        writeln!(file, "</div>");
    }

    if let Some(blog) = &author.blog {
        writeln!(file, "<div>");
        writeln!(file, "<p>Blog: <a href='{}'>{}</a></p>", blog, blog);
        writeln!(file, "</div>");
    }

    writeln!(file, "<div>");
    writeln!(file, "<h2>Blog posts</h2>");
    for entry in index_data {
        if let Some(urls) = author_maps.author_blog_posts.get(&author.name) {
            if !urls.contains(&entry.post_meta.url) {
                continue;
            }
        } else {
            continue;
        }
        writeln!(file, "<div>");
        let title_slug = sanitize::title_to_slug(entry.title.clone());
        writeln!(file, "<p><a href='../p/{}.html'>{}</a></p>", title_slug, entry.title);
        writeln!(file, "</div>");
    }
    writeln!(file, "</div>");

    writeln!(file, "</main>");
    writeln!(file, "</body>");

    Ok(())
}
