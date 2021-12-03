/// Convert a OUT_DIR path into workdir
///
/// ```ignore
/// let dest = std::env::var("OUT_DIR").expect("OUT_DIR env not found");
/// // => "/Users/jason/work/rust-i18n/target/release/build/rust-i18n-cfa390035e3fe523/out"
/// rust_i18n_support::workdir(&dest);
/// // => "/Users/jason/work/rust-i18n"
/// ```
pub fn workdir(dest: &str) -> String {
    let seperator = regex::Regex::new(r"(/target/(.+?)/build/)|(\\target\\(.+?)\\build\\)")
        .expect("Invalid regex");
    let parts = seperator.split(dest).collect::<Vec<_>>();

    if parts.len() < 2 {
        panic!("Parse workdir error, {} not correct.", dest);
    }

    parts[0].to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_workdir() {
        assert_eq!(
            r#"D:\a\rust-i18n\rust-i18n"#,
            crate::workdir(
                r#"D:\a\rust-i18n\rust-i18n\target\debug\build\rust-i18n-04629f744780473d\build-script-build"#
            )
        );

        assert_eq!(
            r#"D:\a\rust-i18n\rust-i18n"#,
            crate::workdir(
                r#"D:\a\rust-i18n\rust-i18n\target\release\build\rust-i18n-04629f744780473d\build-script-build"#
            )
        );

        assert_eq!(
            r#"/Users/jason/work/rust-i18n"#,
            crate::workdir(
                r#"/Users/jason/work/rust-i18n/target/debug/build/rust-i18n-d1612e30e02f745c/out"#
            )
        );

        assert_eq!(
            r#"/Users/jason/work/rust-i18n"#,
            crate::workdir(
                r#"/Users/jason/work/rust-i18n/target/release/build/rust-i18n-d1612e30e02f745c/out"#
            )
        );

        assert_eq!(
            r#"/Users/jason/work/rust-i18n"#,
            crate::workdir(
                r#"/Users/jason/work/rust-i18n/target/foo/build/rust-i18n-d1612e30e02f745c/out"#
            )
        );

        assert_eq!(
            r"/opt/rustwide",
            crate::workdir(
                r"/opt/rustwide/target/x86_64-unknown-linux-gnu/debug/build/rust-i18n-c1b1cee86c09c9f1/out"
            )
        )
    }
}
