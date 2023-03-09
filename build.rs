// https://github.com/longbridgeapp/rust-i18n/blob/v0.1.6/crates/support/src/lib.rs#L9
fn workdir() -> Option<String> {
    if let Ok(pwd) = std::env::var("PWD") {
        return Some(pwd);
    }

    let dest = std::env::var("OUT_DIR");
    if dest.is_err() {
        return None;
    }
    let dest = dest.unwrap();

    let seperator = regex::Regex::new(r"(/target/(.+?)/build/)|(\\target\\(.+?)\\build\\)")
        .expect("Invalid regex");
    let parts = seperator.split(dest.as_str()).collect::<Vec<_>>();

    if parts.len() >= 2 {
        return Some(parts[0].to_string());
    }

    None
}

fn main() {
    let workdir = workdir().unwrap_or("./".to_string());

    let locale_path = format!("{workdir}/**/locales/**/*.yml");
    if let Ok(globs) = glob::glob(&locale_path) {
        for entry in globs {
            let entry = entry.unwrap();
            println!("cargo:rerun-if-changed={}", entry.display());
        }
    }
}
