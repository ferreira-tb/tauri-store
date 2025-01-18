use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<String> {
  fs::read_to_string(path)
    .with_context(|| format!("failed to read file: {}", path.display()).bright_red())
}

pub fn write_file(path: &Path, contents: &str) -> Result<()> {
  let display = path.display();
  fs::write(path, contents)
    .with_context(|| format!("failed to write file: {display}").bright_red())
    .inspect(|()| println!("{}", display.to_string().truecolor(105, 105, 105)))
}
