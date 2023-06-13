use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

mod backend;
pub use backend::Backend;
pub use backend::SimpleBackend;

type Locale = String;
type Value = serde_json::Value;
type Translations = HashMap<Locale, Value>;

pub fn is_debug() -> bool {
    std::env::var("RUST_I18N_DEBUG").unwrap_or_else(|_| "0".to_string()) == "1"
}

/// Merge JSON Values, merge b into a
fn merge_value(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_value(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

// Load locales into flatten key, value HashMap
pub fn load_locales<F: Fn(&str) -> bool>(
    locales_path: &str,
    ignore_if: F,
) -> HashMap<String, HashMap<String, String>> {
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut translations = HashMap::new();

    let path_pattern = format!("{locales_path}/**/*.{{yml,yaml,json,toml}}");

    if is_debug() {
        println!("cargo:i18n-locale={}", &path_pattern);
    }

    // check dir exists
    if !PathBuf::from(locales_path).exists() {
        if is_debug() {
            println!("cargo:i18n-error=path not exists: {}", locales_path);
        }
        return result;
    }

    for entry in globwalk::glob(&path_pattern).expect("Failed to read glob pattern") {
        let entry = entry.unwrap().into_path();
        if is_debug() {
            println!("cargo:i18n-load={}", &entry.display());
        }

        if ignore_if(&entry.display().to_string()) {
            continue;
        }

        let locale = entry
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split('.').last())
            .unwrap();

        let ext = entry.extension().and_then(|s| s.to_str()).unwrap();

        let file = File::open(&entry).expect("Failed to open file");
        let mut reader = std::io::BufReader::new(file);
        let mut content = String::new();

        reader
            .read_to_string(&mut content)
            .expect("Read file failed.");

        let trs = parse_file(&content, ext, locale).expect("Parse file failed.");

        trs.into_iter().for_each(|(k, new_value)| {
            translations
                .entry(k)
                .and_modify(|old_value| merge_value(old_value, &new_value))
                .or_insert(new_value);
        });
    }

    translations.iter().for_each(|(locale, trs)| {
        result.insert(locale.to_string(), flatten_keys(locale, trs));
    });

    result
}

// Parse Translations from file to support multiple formats
fn parse_file(content: &str, ext: &str, locale: &str) -> Result<Translations, String> {
    let result = match ext {
        "yml" | "yaml" => serde_yaml::from_str::<serde_json::Value>(content)
            .map_err(|err| format!("Invalid YAML format, {}", err)),
        "json" => serde_json::from_str::<serde_json::Value>(content)
            .map_err(|err| format!("Invalid JSON format, {}", err)),
        "toml" => toml::from_str::<serde_json::Value>(content)
            .map_err(|err| format!("Invalid TOML format, {}", err)),
        _ => Err("Invalid file extension".into()),
    };

    match result {
        Ok(v) => Ok(Translations::from([(locale.to_string(), v)])),
        Err(e) => Err(e),
    }
}

fn flatten_keys(prefix: &str, trs: &Value) -> HashMap<String, String> {
    let mut v = HashMap::<String, String>::new();
    let prefix = prefix.to_string();

    match &trs {
        serde_json::Value::String(s) => {
            v.insert(prefix, s.to_string());
        }
        serde_json::Value::Object(o) => {
            for (k, vv) in o {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                v.extend(flatten_keys(key.as_str(), vv));
            }
        }
        serde_json::Value::Null => {
            v.insert(prefix, "".into());
        }
        serde_json::Value::Bool(s) => {
            v.insert(prefix, format!("{}", s));
        }
        serde_json::Value::Number(s) => {
            v.insert(prefix, format!("{}", s));
        }
        serde_json::Value::Array(_) => {
            v.insert(prefix, "".into());
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::{merge_value, parse_file};
    use std::path::PathBuf;

    #[test]
    fn test_merge_value() {
        let a = serde_json::from_str::<serde_json::Value>(
            r#"{"foo": "Foo", "dar": { "a": "1", "b": "2" }}"#,
        )
        .unwrap();
        let b = serde_json::from_str::<serde_json::Value>(
            r#"{"foo": "Foo1", "bar": "Bar", "dar": { "b": "21" }}"#,
        )
        .unwrap();

        let mut c = a.clone();
        merge_value(&mut c, &b);

        assert_eq!(c["foo"], "Foo1");
        assert_eq!(c["bar"], "Bar");
        assert_eq!(c["dar"]["a"], "1");
        assert_eq!(c["dar"]["b"], "21");
    }

    #[test]
    fn test_parse_file_in_yaml() {
        let content = "foo: Foo\nbar: Bar";
        let mut trs = parse_file(content, "yml", "en").expect("Should ok");
        assert_eq!(trs["en"]["foo"], "Foo");
        assert_eq!(trs["en"]["bar"], "Bar");

        trs = parse_file(content, "yaml", "en").expect("Should ok");
        assert_eq!(trs["en"]["foo"], "Foo");

        trs = parse_file(content, "yml", "zh-CN").expect("Should ok");
        assert_eq!(trs["zh-CN"]["foo"], "Foo");

        parse_file(content, "foo", "en").expect_err("Should error");
        parse_file("invalid content", "yml", "en").expect_err("Should error");
    }

    #[test]
    fn test_parse_file_in_json() {
        let content = r#"
        {
            "foo": "Foo",
            "bar": "Bar"
        }
        "#;
        let trs = parse_file(content, "json", "en").expect("Should ok");
        assert_eq!(trs["en"]["foo"], "Foo");
        assert_eq!(trs["en"]["bar"], "Bar");
    }

    #[test]
    fn test_parse_file_in_toml() {
        let content = r#"
        foo = "Foo"
        bar = "Bar"
        "#;
        let trs = parse_file(content, "toml", "en").expect("Should ok");
        assert_eq!(trs["en"]["foo"], "Foo");
        assert_eq!(trs["en"]["bar"], "Bar");
    }
}
