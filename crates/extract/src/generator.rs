use crate::extractor::Message;
use rust_i18n_support::{load_locales, Translations};
use std::collections::HashMap;
use std::io::prelude::*;
use std::path::Path;

pub fn generate<'a, P: AsRef<Path>>(
    output: P,
    locale: &str,
    messages: impl IntoIterator<Item = &'a Message>,
) -> ::std::io::Result<()> {
    let output_filename = format!("TODO.{}.yml", locale);
    let output_path = format!("{}", output.as_ref().display());
    let ignore_file = |fname: &str| fname.ends_with(&output_filename);
    let old_translations = load_locales(&output_path, ignore_file);

    let mut new_translations: Translations = HashMap::new();
    let mut new_values: HashMap<String, String> = HashMap::new();

    for m in messages {
        let key = format!("{}.{}", locale, m.key);

        if !m.locations.is_empty() {
            for _l in &m.locations {
                // TODO: write file and line as YAML comment
            }
        }

        if old_translations.get(&key).is_some() {
            continue;
        }

        new_values.entry(m.key.clone()).or_insert_with(|| "".into());
    }

    new_translations.insert(locale.to_string(), serde_json::to_value(&new_values)?);

    if new_values.is_empty() {
        return Ok(());
    }

    let output_file = std::path::Path::new(output.as_ref()).join(output_filename);
    println!("Writing {}", output_file.display());
    let mut output = ::std::fs::File::create(output_file)?;
    writeln!(
        output,
        "{}",
        serde_yaml::to_string(&new_translations).unwrap()
    )
    .expect("Write YAML file error");

    Ok(())
}
