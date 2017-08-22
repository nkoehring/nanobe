use logger;
use glob::glob;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write, BufReader};
use comrak::{markdown_to_html, ComrakOptions};


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

    for path in input_files {
      let file_path = String::from(path.to_string_lossy());
      let file_name = String::from(path.file_name().unwrap().to_string_lossy());

      if let Ok(file) = File::open(path) {
        let mut file_reader = BufReader::new(file);
        let mut buffer = String::new();
        let bytes_read = file_reader.read_to_string(&mut buffer);

        info!(logger, "{} bytes read from {} into buffer", bytes_read.unwrap_or(0), file_path);
        let html = markdown_to_html(&buffer, &ComrakOptions::default());

        let mut output_path = PathBuf::from(output_dir);
        output_path.push(file_name);
        output_path.set_extension("html");

        if let Ok(mut output_file) = File::create(&output_path) {
          let bytes_written = output_file.write(html.as_bytes());
          info!(logger, "{} bytes written to {:?}", bytes_written.unwrap_or(0), output_path);
        } else {
          error!(logger, "couldn't open output file '{:?}'", output_path)
        };

      } else {
        error!(logger, "Wasn't able to read '{}'", file_name);
      }
    }
}
