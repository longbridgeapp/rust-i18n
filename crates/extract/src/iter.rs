use anyhow::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn iter_crate<F>(src_path: &str, mut callback: F) -> Result<(), Error>
where
    F: FnMut(&PathBuf, &str) -> Result<(), Error>,
{
    let pattern = format!("{}/**/*.rs", src_path);
    for entry in glob::glob(&pattern)? {
        let entry = entry.unwrap();
        let file_path = entry.as_path();

        let mut s = String::new();
        let mut f = File::open(&file_path).expect("Failed to open file");
        f.read_to_string(&mut s).expect("Failed to read file");

        callback(&entry, &s)?;
    }

    Ok(())
}
