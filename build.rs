// https://github.com/longbridgeapp/rust-i18n/blob/v0.1.6/crates/support/src/lib.rs#L9
fn workdir(dest: &str) -> String {
    let seperator = regex::Regex::new(r"(/target/(.+?)/build/)|(\\target\\(.+?)\\build\\)")
        .expect("Invalid regex");
    let parts = seperator.split(dest).collect::<Vec<_>>();

    if parts.len() < 2 {
        panic!("Parse workdir error, {} not correct.", dest);
    }

    parts[0].to_string()
}

fn find_all_yaml_for_cargo_cache() {
    let dest = std::env::var("OUT_DIR").expect("OUT_DIR env not found");
    let workdir = workdir(&dest);
    let locale_path = format!("{}/**/locales/**/*.yml", workdir);

    for entry in glob::glob(&locale_path).expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.display());
    }
}

fn main() {
    find_all_yaml_for_cargo_cache();
}
