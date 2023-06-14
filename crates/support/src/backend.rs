use std::collections::HashMap;

/// I18n backend trait
pub trait Backend: Send + Sync + 'static {
    /// Return the available locales
    fn available_locales(&self) -> Vec<String>;
    /// Get the translation for the given locale and key
    fn translate(&self, locale: &str, key: &str) -> Option<String>;
}

trait BackendExt: Backend {
    /// Extend backend to add more translations
    fn extend<T: Backend>(self, other: T) -> CombinedBackend<Self, T>
    where
        Self: Sized,
    {
        CombinedBackend(self, other)
    }
}

struct CombinedBackend<A, B>(A, B);

impl<A, B> Backend for CombinedBackend<A, B>
where
    A: Backend,
    B: Backend,
{
    fn available_locales(&self) -> Vec<String> {
        let mut available_locales = self.0.available_locales();
        for locale in self.1.available_locales() {
            if !available_locales.contains(&locale) {
                available_locales.push(locale);
            }
        }
        available_locales
    }

    fn translate(&self, locale: &str, key: &str) -> Option<String> {
        self.1
            .translate(locale, key)
            .or_else(|| self.0.translate(locale, key))
    }
}

/// Simple KeyValue storage backend
pub struct SimpleBackend {
    /// All translations key is flatten key, like `en.hello.world`
    translations: HashMap<String, String>,
    /// All available locales
    locales: HashMap<String, bool>,
}

impl SimpleBackend {
    /// Create a new SimpleBackend.
    ///
    /// In `translations` HashMap, the key is flatten key, like `en.hello.world`.
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

    /// Add more translations for the given locale.
    ///
    /// ```ignore
    /// let trs = HashMap::<String, String>::new();
    /// trs.insert("hello".into(), "Hello".into());
    /// trs.insert("foo".into(), "Foo bar".into());
    /// backend.add_translations("en", &data);
    /// ```
    pub fn add_translations(&mut self, locale: &str, data: &HashMap<String, String>) {
        data.iter().for_each(|(k, v)| {
            let k = format!("{}.{}", locale, k);
            self.translations.insert(k, v.to_string());
        });
        self.locales.insert(locale.into(), true);
    }
}

impl Backend for SimpleBackend {
    fn available_locales(&self) -> Vec<String> {
        let mut locales = self
            .locales
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<_>>();
        locales.sort();
        locales
    }

    fn translate(&self, locale: &str, key: &str) -> Option<String> {
        let flatten_key = format!("{}.{}", locale, key);
        self.translations.get(&flatten_key).map(|v| v.to_string())
    }
}

impl BackendExt for SimpleBackend {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::SimpleBackend;
    use super::{Backend, BackendExt};

    #[test]
    fn test_simple_backend() {
        let mut backend = SimpleBackend::new(HashMap::new(), vec![]);
        let mut data = HashMap::<String, String>::new();
        data.insert("hello".into(), "Hello".into());
        data.insert("foo".into(), "Foo bar".into());
        backend.add_translations("en", &data);

        let mut data_cn = HashMap::<String, String>::new();
        data_cn.insert("hello".into(), "你好".into());
        data_cn.insert("foo".into(), "Foo 测试".into());
        backend.add_translations("zh-CN", &data_cn);

        assert_eq!(backend.translate("en", "hello"), Some("Hello".into()));
        assert_eq!(backend.translate("en", "foo"), Some("Foo bar".into()));
        assert_eq!(backend.translate("zh-CN", "hello"), Some("你好".into()));
        assert_eq!(backend.translate("zh-CN", "foo"), Some("Foo 测试".into()));

        assert_eq!(backend.available_locales(), vec!["en", "zh-CN"]);
    }

    #[test]
    fn test_combined_backend() {
        let mut backend = SimpleBackend::new(HashMap::new(), vec![]);
        let mut data = HashMap::<String, String>::new();
        data.insert("hello".into(), "Hello".into());
        data.insert("foo".into(), "Foo bar".into());
        backend.add_translations("en", &data);

        let mut data_cn = HashMap::<String, String>::new();
        data_cn.insert("hello".into(), "你好".into());
        data_cn.insert("foo".into(), "Foo 测试".into());
        backend.add_translations("zh-CN", &data_cn);

        let mut backend2 = SimpleBackend::new(HashMap::new(), vec![]);
        let mut data2 = HashMap::<String, String>::new();
        data2.insert("hello".into(), "Hello2".into());
        backend2.add_translations("en", &data2);

        let mut data_cn2 = HashMap::<String, String>::new();
        data_cn2.insert("hello".into(), "你好2".into());
        backend2.add_translations("zh-CN", &data_cn2);

        let combined = backend.extend(backend2);
        assert_eq!(combined.translate("en", "hello"), Some("Hello2".into()));
        assert_eq!(combined.translate("zh-CN", "hello"), Some("你好2".into()));

        assert_eq!(combined.available_locales(), vec!["en", "zh-CN"]);
    }
}
