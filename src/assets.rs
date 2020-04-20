use log::info;
use std::fs;
use anyhow::{Result, Context};
use std::path::PathBuf;

pub struct AssetDirs {
    pub css_dir: PathBuf,
}

pub static RESET_CSS_FILE: &'static str = "reset.css";
pub static MAIN_CSS_FILE: &'static str = "main.css";
pub static BLOG_CSS_FILE: &'static str = "blog.css";

static RESET_CSS_CONTENTS: &'static str = include_str!("css/reset.css");
static MAIN_CSS_CONTENTS: &'static str = include_str!("css/main.css");
static BLOG_CSS_CONTENTS: &'static str = include_str!("css/blog.css");

pub fn copy(dirs: &AssetDirs) -> Result<()> {
    let css_dir = &dirs.css_dir;
    let reset_file = &css_dir.join(RESET_CSS_FILE);
    let main_file = &css_dir.join(MAIN_CSS_FILE);
    let blog_file = &css_dir.join(BLOG_CSS_FILE);

    fs::create_dir_all(css_dir)
        .context("creating css dir")?;

    fs::write(reset_file, RESET_CSS_CONTENTS)
        .context("writing css")?;
    info!("created {}", reset_file.display());
    fs::write(main_file, MAIN_CSS_CONTENTS)
        .context("writing css")?;
    info!("created {}", main_file.display());
    fs::write(blog_file, BLOG_CSS_CONTENTS)
        .context("writing css")?;
    info!("created {}", blog_file.display());

    Ok(())
}
