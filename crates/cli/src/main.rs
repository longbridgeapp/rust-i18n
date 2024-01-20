use anyhow::Error;
use clap::{Args, Parser};

use std::{collections::HashMap, path::Path};

use rust_i18n_extract::{extractor, generator, iter};
mod config;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    I18n(I18nArgs),
}

#[derive(Args)]
#[command(author, version)]
// #[command(propagate_version = true)]
/// Rust I18n command to help you extract all untranslated texts from source code.
///
/// It will iterate all Rust files in the source directory and extract all untranslated texts
/// that used `t!` macro.
/// Then it will generate a YAML file and merge with the existing translations.
///
/// https://github.com/longbridgeapp/rust-i18n
struct I18nArgs {
    /// Extract all untranslated I18n texts from source code
    #[arg(default_value = "./")]
    source: Option<String>,
}

fn main() -> Result<(), Error> {
    let CargoCli::I18n(args) = CargoCli::parse();

    let mut results = HashMap::new();

    let source_path = args.source.expect("Missing source path");

    let cfg = config::load(std::path::Path::new(&source_path))?;

    iter::iter_crate(&source_path, |path, source| {
        extractor::extract(&mut results, path, source)
    })?;

    let mut messages: Vec<_> = results.iter().collect();
    messages.sort_by_key(|(_k, m)| m.index);

    let mut has_error = false;

    let output_path = Path::new(&source_path).join(&cfg.load_path);

    let result = generator::generate(output_path, &cfg.available_locales, messages.clone());
    if result.is_err() {
        has_error = true;
    }

    if has_error {
        std::process::exit(1);
    }

    Ok(())
}
