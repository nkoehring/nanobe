use logger;
use glob::glob;
use std::path::{Path, PathBuf};
use std::fs::{create_dir_all, File};
use std::io::{Read, BufReader, Error};
use std::sync::Arc;
use rayon::prelude::*;

use model::website::Website;


fn render_file(input_path: &PathBuf, output_dir: &Path, website: Arc<Website>) -> Result<String, Error> {
    let file_name = String::from(input_path.file_name().unwrap().to_string_lossy());
    let file = File::open(input_path)?;

    let mut file_reader = BufReader::new(file);
    let mut buffer = String::new();

    let _ = file_reader.read_to_string(&mut buffer);
    (*website).render_article(&buffer, &file_name, output_dir)
}


pub fn build(input_dir: &str, output_dir: &str) {
    let logger = logger::build();
    let pattern = format!("{}/*.md", input_dir);
    let input_path = Path::new(input_dir);
    let mut output_path = PathBuf::new();
    output_path.push(output_dir);
    output_path.push("articles");

    if !input_path.exists() || !input_path.is_dir() {
        error!(logger, "source '{}' doesn't exist or is not a directory", input_path.display());
        return;
    }

    match create_dir_all(output_path.as_path()) {
        Ok(_) => info!(logger, "created output directory tree"),
        Err(err) => error!(logger, "{}", err)
    }

    info!(logger, "traversing source directory '{}'", input_path.display());
    let input_files: Vec<_> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
    info!(logger, "found {} items in '{}'", input_files.len(), input_path.display());

    let website = Arc::new(Website::default());
    input_files.par_iter().for_each(|path| {
      match render_file(path, output_path.as_path(), website.clone()) {
        Ok(file_name) => info!(logger, "Rendered {}", file_name),
        Err(err) => error!(logger, "{}", err)
      }
    })
}
