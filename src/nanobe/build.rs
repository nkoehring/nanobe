use logger;
use glob::glob;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, BufReader, Error};
use rayon::prelude::*;

use nanobe::article::Article;


fn render_file(input_path: &PathBuf, output_dir: &Path) -> Result<String, Error> {
  let file_name = String::from(input_path.file_name().unwrap().to_string_lossy());
  let file = File::open(input_path)?;
  let mut file_reader = BufReader::new(file);
  let mut buffer = String::new();
  let _ = file_reader.read_to_string(&mut buffer);
  let article = Article::from_markdown(&buffer);
  article.dump(&file_name, output_dir)
}


pub fn build(input_dir: &str, output_dir: &str) {
    let logger = logger::build();
    let pattern = format!("{}/*.md", input_dir);
    let input_dir = Path::new(input_dir);
    let output_dir = Path::new(output_dir);

    if !input_dir.exists() || !input_dir.is_dir() {
        error!(logger, "source '{}' doesn't exist or is not a directory", input_dir.display());
        return;
    }
    if !output_dir.exists() || !output_dir.is_dir() {
        error!(logger, "target '{}' doesn't exist or is not a directory", output_dir.display());
        return;
    }

    info!(logger, "traversing source directory '{}'", input_dir.display());
    let input_files: Vec<_> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
    info!(logger, "found {} items in '{}'", input_files.len(), input_dir.display());

    input_files.par_iter().for_each(|path| {
      match render_file(path, output_dir) {
        Ok(file_name) => info!(logger, "Rendered {}", file_name),
        Err(err) => error!(logger, "{}", err)
      }
    })
}
