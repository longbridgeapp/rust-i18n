use std::collections::HashMap;

// I18n backend trait
pub trait Backend: Send + Sync + 'static {
    // Insert the translations for the given locale
    fn store(&mut self, locale: &str, data: &HashMap<String, String>);
    // Return the available locales
    fn available_locales(&self) -> Vec<String>;
    // Lookup the translation for the given locale and key
    fn lookup(&self, locale: &str, key: &str) -> Option<String>;
}

// Simple KeyValue storage backend
pub struct SimpleBackend {
    pub translations: HashMap<String, String>,
    pub locales: HashMap<String, bool>,
}

impl SimpleBackend {
    /// Create a new SimpleBackend.
    ///
    /// ```ignore
    /// let trs = HashMap::<String, String>::new();
    /// trs.insert("en.hello".into(), "Hello".into());
    /// trs.insert("en.foo".into(), "Foo bar".into());
    /// trs.insert("zh-CN.hello".into(), "你好".into());
    /// trs.insert("zh-CN.foo".into(), "Foo 测试".into());
    /// let locales = vec!["en", "zh-CN"];
    /// let backend = SimpleBackend::new(trs, locales);
    /// ```
    pub fn new(translations: HashMap<&str, &str>, locales: Vec<&str>) -> Self {
        SimpleBackend {
            locales: locales.iter().map(|l| (l.to_string(), true)).collect(),
            translations: translations
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

impl Backend for SimpleBackend {
    /// Insert the translations for the given locale.
    ///
    /// ```ignore
    /// let trs = HashMap::<String, String>::new();
    /// trs.insert("hello".into(), "Hello".into());
    /// trs.insert("foo".into(), "Foo bar".into());
    /// backend.store("en", &data);
    /// ```
    fn store(&mut self, locale: &str, data: &HashMap<String, String>) {
        data.iter().for_each(|(k, v)| {
            let k = format!("{}.{}", locale, k);
            self.translations.insert(k, v.to_string());
        });
        self.locales.insert(locale.into(), true);
    }

    fn available_locales(&self) -> Vec<String> {
        self.locales.keys().map(|k| k.to_string()).collect()
    }

    fn lookup(&self, locale: &str, key: &str) -> Option<String> {
        let flatten_key = format!("{}.{}", locale, key);
        self.translations.get(&flatten_key).map(|v| v.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::SimpleBackend;

    #[test]
    fn test_simple_backend() {
        let backend = SimpleBackend::new(HashMap::new(), vec![]);
        let mut data = HashMap::<String, String>::new();
        data.insert("hello".into(), "Hello".into());
        data.insert("foo".into(), "Foo bar".into());
        backend.store("en", &data);

        let mut data_cn = HashMap::<String, String>::new();
        data.insert("hello".into(), "你好".into());
        data.insert("foo".into(), "Foo 测试".into());
        backend.store("zh-CN", &data_cn);

        assert_eq!(backend.lookup("en", "hello"), Some("Hello".into()));
        assert_eq!(backend.lookup("en", "foo"), Some("Foo bar".into()));
        assert_eq!(backend.lookup("zh-CN", "hello"), Some("你好".into()));
        assert_eq!(backend.lookup("zh-CN", "foo"), Some("Foo 测试".into()));

        assert_eq!(backend.available_locales(), vec!["en", "zh-CN"]);
    }
}
