use crate::sanitize;
use crate::config::Category;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use log::info;
use std::io::Write;
use anyhow::{Result, Context};
use std::path::Path;
use std::fs::{self, File};
use crate::assets::AssetDirs;
use crate::assets::{RESET_CSS_FILE, MAIN_CSS_FILE, BLOG_CSS_FILE};
use crate::render;
use crate::config::BlogPost;
use crate::author::AuthorMaps;

pub struct IndexEntry {
    pub post_meta: BlogPost,
    pub title: String,
    pub file_name: String,
}

static TITLE: &'static str = "The Rust Docuverse";

pub fn write(dir: &Path, assets: &AssetDirs, data: Vec<IndexEntry>, authors: AuthorMaps) -> Result<()> {
    fs::create_dir_all(dir)?;
    let index_file = dir.join("index.html");
    let mut file = File::create(&index_file)
        .context("opening index file")?;

    writeln!(file, "<!doctype html>");
    writeln!(file, "<html lang='en'>");

    let header_meta = render::HeaderMeta {
        title: Some("The Rust Docuniverse".to_string()),
    };
        
    render::render_head(&mut file, assets, &header_meta);
    write_body(&mut file, data, authors)?;

    writeln!(file, "</html>");

    info!("index written to {}", index_file.display());
    
    Ok(())
}

type Categories = BTreeMap<Category, Vec<IndexEntry>>;

fn write_body(file: &mut File, entries: Vec<IndexEntry>, authors: AuthorMaps) -> Result<()> {
    let categories = categorize(entries);

    writeln!(file, "<body>");
    writeln!(file, "<main>");
    writeln!(file, "<h1>{}</h1>", TITLE);
    for (category, entries) in categories {
        writeln!(file, "<section>");
        writeln!(file, "<h2>{}</h2>", category);
        for entry in entries {
            let title = &entry.title;
            let file_name = &entry.file_name;
            writeln!(file, "<p>");
            writeln!(file, "<a href='./p/{}.html'>{}</a>",
                     file_name, title);
            maybe_write_author(file, &entry, &authors)?;
            writeln!(file, "</p>");
        }
        writeln!(file, "</section>");
    }
    writeln!(file, "</main>");
    writeln!(file, "</body>");

    Ok(())
}

fn maybe_write_author(file: &mut File, entry: &IndexEntry, authors: &AuthorMaps) -> Result<()> {
    if let Some(name) = authors.blog_post_author.get(&entry.post_meta.url) {
        writeln!(file, "<span>");
        let name_slug = sanitize::name_to_slug(name.to_string());
        let link = format!("./a/{}.html", name_slug);
        writeln!(file, "by <a href='{}'>{}</a>", link, name);
        writeln!(file, "</span>");
    }
    Ok(())
}

fn categorize(entries: Vec<IndexEntry>) -> Categories {
    let mut map = BTreeMap::new();

    for entry in entries {
        match map.entry(entry.post_meta.category.clone()) {
            Entry::Vacant(v) => {
                v.insert(vec![entry]);
            }
            Entry::Occupied(mut v) => {
                v.get_mut().push(entry);
            }
        }
    }

    map
}
