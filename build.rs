// https://github.com/longbridgeapp/rust-i18n/blob/v0.1.6/crates/support/src/lib.rs#L9
fn workdir() -> Option<String> {
    // Do not panic if OUT_DIR or workdir not correct.
    // Because in cargo install OUT_DIR is:
    // /var/folders/qb/.../T/cargo-installcBf9L5/release/build/rust-i18n-xxx/build-script-build
    let dest = std::env::var("OUT_DIR");
    if dest.is_err() {
        return None;
    }
    let dest = dest.unwrap();

    let seperator = regex::Regex::new(r"(/target/(.+?)/build/)|(\\target\\(.+?)\\build\\)")
        .expect("Invalid regex");
    let parts = seperator.split(&dest).collect::<Vec<_>>();

    if parts.len() >= 2 {
        return Some(parts[0].to_string());
    }

    None
}

fn main() {
    let workdir = workdir();
    let workdir = workdir.unwrap_or_else(|| {
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is empty")
    });
    let locale_path = format!("{workdir}/**/locales/**/*.yml");

    for entry in glob::glob(&locale_path).expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.display());
    }
}
