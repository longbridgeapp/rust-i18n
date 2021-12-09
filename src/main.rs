use anyhow::Error;
use clap::{App, Arg, SubCommand};

use std::collections::HashMap;

use rust_i18n_extract::{extractor, generator, iter};

fn main() -> Result<(), Error> {
    let extract_command = SubCommand::with_name("extract")
        .about("Extracts strings from source files")
        .arg(
            Arg::with_name("output")
                .short("o")
                .default_value("./locales")
                .help("Path for output locales YAML files."),
        )
        .arg(
            Arg::with_name("locale")
                .short("l")
                .help("Source locale")
                .default_value("en"),
        )
        .arg(
            Arg::with_name("source")
                .help("Path of the Rust source code")
                .default_value("./"),
        );

    let app = App::new("i18n")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Rust I18n is a crate for loading localized text from a set of YAML mapping files. The mappings are converted into data readable by Rust programs at compile time, and then localized text can be loaded by simply calling the provided `t!` macro.")
        .subcommand(extract_command)
        .get_matches();

    let mut results = HashMap::new();

    match app.subcommand() {
        ("extract", Some(sub_m)) => {
            let source_path = sub_m.value_of("source").expect("Missing source path");
            let output_path = sub_m.value_of("output").expect("Missing output path");
            let source_locale = sub_m.value_of("locale").expect("Missing source locale");

            iter::iter_crate(source_path, |path, source| {
                extractor::extract(&mut results, path, source)
            })?;

            let mut messages: Vec<_> = results.values().collect();
            messages.sort_by_key(|m| m.index);

            let result = generator::generate(output_path, source_locale, messages);
            if result.is_err() {
                std::process::exit(1);
            }
        }
        _ => {}
    }

    Ok(())
}
