use std::path::Path;
use crate::assets::AssetDirs;
use crate::assets::{RESET_CSS_FILE, MAIN_CSS_FILE, BLOG_CSS_FILE};
use std::iter::FromIterator;
use anyhow::Result;
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

pub fn write_pages(dir: &Path, authors: &Vec<Author>, assets: &AssetDirs, data: Vec<IndexEntry>, author_maps: AuthorMaps) -> Result<()> {
    panic!()
}
