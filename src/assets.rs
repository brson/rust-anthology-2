use std::fs;
use anyhow::{Result, Context};
use std::path::PathBuf;

pub struct AssetDirs {
    pub css_dir: PathBuf,
}

pub static RESET_FILE: &'static str = "reset.css";
pub static NINJA_FILE: &'static str = "ninja-main.css";

pub static RESET_CONTENTS: &'static str = include_str!("css/reset.css");
pub static NINJA_CONTENTS: &'static str = include_str!("css/ninja-main.css");

pub fn copy(dirs: &AssetDirs) -> Result<()> {
    let css_dir = &dirs.css_dir;
    let reset_file = &css_dir.join(RESET_FILE);
    let ninja_file = &css_dir.join(NINJA_FILE);

    fs::create_dir_all(css_dir)
        .context("creating css dir")?;
    fs::write(reset_file, RESET_CONTENTS)
        .context("writing css")?;
    fs::write(ninja_file, NINJA_CONTENTS)
        .context("writing css")?;

    Ok(())
}
