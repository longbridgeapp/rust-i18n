// https://github.com/longbridgeapp/rust-i18n/blob/v0.1.6/crates/support/src/lib.rs#L9
fn workdir(dest: &str) -> Option<String> {
    let seperator = regex::Regex::new(r"(/target/(.+?)/build/)|(\\target\\(.+?)\\build\\)")
        .expect("Invalid regex");
    let parts = seperator.split(dest).collect::<Vec<_>>();

    if parts.len() >= 2 {
        return Some(parts[0].to_string());
    }

    None
}

fn find_all_yaml_for_cargo_cache() {
    // Do not panic if OUT_DIR or workdir not correct.
    // Because in cargo install OUT_DIR is:
    // /var/folders/qb/.../T/cargo-installcBf9L5/release/build/rust-i18n-xxx/build-script-build
    let dest = std::env::var("OUT_DIR");
    if dest.is_err() {
        return;
    }
    let dest = dest.unwrap();

    let workdir = workdir(&dest);
    if workdir.is_none() {
        return;
    }

    let workdir = workdir.unwrap();
    let locale_path = format!("{}/**/locales/**/*.yml", workdir);

    for entry in glob::glob(&locale_path).expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.display());
    }
}

fn main() {
    find_all_yaml_for_cargo_cache();
}
