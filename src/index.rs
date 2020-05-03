use log::info;
use std::io::Write;
use anyhow::{Result, Context};
use std::path::Path;
use std::fs::{self, File};
use crate::assets::AssetDirs;
use crate::assets::{RESET_CSS_FILE, MAIN_CSS_FILE, BLOG_CSS_FILE};
use crate::render;
use crate::config::BlogPost;

pub struct IndexEntry {
    pub post_meta: BlogPost,
    pub title: String,
    pub file_name: String,
}

static TITLE: &'static str = "The Rust Docuverse";

pub fn write(dir: &Path, assets: &AssetDirs, data: Vec<IndexEntry>) -> Result<()> {
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
    write_body(&mut file, data)?;

    writeln!(file, "</html>");

    info!("index written to {}", index_file.display());
    
    Ok(())
}

fn write_body(file: &mut File, entries: Vec<IndexEntry>) -> Result<()> {
    writeln!(file, "<body>");
    writeln!(file, "<main>");
    writeln!(file, "<h1>{}</h1>", TITLE);
    for entry in entries {
        let title = entry.title;
        let file_name = entry.file_name;
        writeln!(file, "<p>");
        writeln!(file, "<a href='./p/{}.html'>{}</a>",
                 file_name, title);
        writeln!(file, "</p>");
    }
    writeln!(file, "</main>");
    writeln!(file, "</body>");

    Ok(())
}
