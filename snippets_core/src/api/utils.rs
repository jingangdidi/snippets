use std::env::current_exe;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::error::MyError;

/// save file
/// https://users.rust-lang.org/t/write-to-normal-or-gzip-file-transparently/35561/2
pub fn my_writer(file: &Path) -> Result<Box<dyn Write>, MyError> {
    let created_file = File::create(file).map_err(|e| MyError::CreateFileError{file: file.to_str().unwrap().to_string(), error: e})?;
    //Ok(Box::new(BufWriter::with_capacity(128 * 1024, created_file)))
    Ok(Box::new(BufWriter::new(created_file)))
}

/// get all *.snippets files from current path or binary file path
pub fn get_snippet_files() -> Result<Vec<PathBuf>, MyError> {
    let mut files: Vec<PathBuf> = Vec::new();
    // search from current path
    let current_path = Path::new("./");
    files.extend(get_snippets(current_path));
    // search from binary file path
    if files.is_empty() {
        if let Ok(mut binary_path) = current_exe() {
            if binary_path.pop() { // Truncates binary_path to parent
                files.extend(get_snippets(&binary_path));
            }
        }
    }
    Ok(files)
}

/// get all *.snippets from path
fn get_snippets(inpath: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    if let Ok(dirs) = inpath.read_dir() {
        for i in dirs {
            if let Ok(entry) = i {
                let tmp_path = entry.path();
                if tmp_path.is_file() {
                    if let Some(ext) = tmp_path.extension() {
                        if ext == "snippets" {
                            files.push(tmp_path);
                        }
                    }
                }
            }
        }
    }
    files
}

/// Computes the cosine similarity between two tone Tensor
/// https://en.wikipedia.org/wiki/Cosine_similarity
/// https://github.com/gaspiman/cosine_similarity/blob/master/cosine.go
pub fn cosine_similarity(vec_a: &Vec<f32>, vec_b: &Vec<f32>) -> Result<f32, MyError> {
    let mut ab: f32 = 0.0;
    let mut sum_a: f32 = 0.0;
    let mut sum_b: f32 = 0.0;
    for (i, j) in vec_a.iter().zip(vec_b.iter()) {
        ab += i * j;
        sum_a += i.powf(2.0);
        sum_b += j.powf(2.0);
    }

    Ok(ab / (sum_a.sqrt() * sum_b.sqrt()))
}

