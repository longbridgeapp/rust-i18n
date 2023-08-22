use crate::extractor::Message;
use rust_i18n_support::load_locales;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

type Translations = HashMap<String, HashMap<String, String>>;

pub fn generate<'a, P: AsRef<Path>>(
    output: P,
    all_locales: &Vec<String>,
    messages: impl IntoIterator<Item = &'a Message> + Clone,
) -> Result<()> {
    let mut trs = Translations::new();
    let filename = "TODO.yml";

    for locale in all_locales {
        println!("Checking [{}] and generating untranslated texts...", locale);

        // ~/work/my-project/locales
        let output_path = output.as_ref().display().to_string();

        let ignore_file = |fname: &str| fname.ends_with(&filename);
        let data = load_locales(&output_path, ignore_file);

        for m in messages.clone() {
            if !m.locations.is_empty() {
                for _l in &m.locations {
                    // TODO: write file and line as YAML comment
                    // Reason: serde_yaml not support write comments
                    // https://github.com/dtolnay/serde-yaml/issues/145
                }
            }

            if let Some(trs) = data.get(locale) {
                if trs.get(&m.key).is_some() {
                    continue;
                }
            }

            let value = m.key.split('.').last().unwrap_or_default();

            trs.entry(m.key.clone())
                .or_insert_with(HashMap::new)
                .insert(locale.to_string(), value.to_string());
        }
    }

    if trs.is_empty() {
        println!("All thing done.\n");

        return Ok(());
    }

    eprintln!("Found {} new texts need to translate.", trs.len());
    eprintln!("----------------------------------------");
    eprintln!("Writing to {}\n", filename);

    write_file(&output, &filename, &trs)?;

    // Finally, return error for let CI fail
    let err = std::io::Error::new(std::io::ErrorKind::Other, "");
    Err(err)
}

fn write_file<P: AsRef<Path>>(
    output: &P,
    filename: &str,
    translations: &Translations,
) -> Result<()> {
    let output_file = std::path::Path::new(output.as_ref()).join(String::from(filename));
    let folder = output_file.parent().unwrap();

    // Ensure create folder
    if !folder.exists() {
        std::fs::create_dir_all(folder).unwrap();
    }

    let mut output = ::std::fs::File::create(&output_file)
        .unwrap_or_else(|_| panic!("Unable to create {} file", &output_file.display()));

    writeln!(output, "{}", serde_yaml::to_string(&translations).unwrap())
        .expect("Write YAML file error");

    Ok(())
}
