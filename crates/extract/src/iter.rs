use anyhow::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn iter_crate<F>(src_path: &str, mut callback: F) -> Result<(), Error>
where
    F: FnMut(&PathBuf, &str) -> Result<(), Error>,
{
    let src_path = src_path.trim_end_matches('/');

    let mut walker = ignore::WalkBuilder::new(src_path);
    walker
        .skip_stdout(true)
        .parents(true)
        .git_ignore(true)
        .follow_links(false);

    for result in walker.build() {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }

                if path.extension() != Some("rs".as_ref()) {
                    continue;
                }

                let filepath = String::from(path.to_str().unwrap());

                let mut s = String::new();
                let mut f = File::open(&filepath).expect("Failed to open file");
                f.read_to_string(&mut s).expect("Failed to read file");

                callback(&PathBuf::from(filepath), &s)?;
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
    Ok(())
}
