use logger;
use glob::glob;
use std::path::Path;


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
}
