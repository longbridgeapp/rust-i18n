use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct I18nAttrs {
    #[serde(default = "minify_key")]
    pub minify_key: bool,
    #[serde(default = "minify_key_len")]
    pub minify_key_len: usize,
    #[serde(default = "minify_key_prefix")]
    pub minify_key_prefix: String,
    #[serde(default = "minify_key_thresh")]
    pub minify_key_thresh: usize,
}

impl I18nAttrs {
    pub fn new() -> Self {
        Self {
            minify_key: rust_i18n_support::DEFAULT_MINIFY_KEY,
            minify_key_len: rust_i18n_support::DEFAULT_MINIFY_KEY_LEN,
            minify_key_prefix: rust_i18n_support::DEFAULT_MINIFY_KEY_PREFIX.to_string(),
            minify_key_thresh: rust_i18n_support::DEFAULT_MINIFY_KEY_THRESH,
        }
    }
}

impl Default for I18nAttrs {
    fn default() -> Self {
        Self::new()
    }
}

fn minify_key() -> bool {
    I18nAttrs::default().minify_key
}

fn minify_key_len() -> usize {
    I18nAttrs::default().minify_key_len
}

fn minify_key_prefix() -> String {
    I18nAttrs::default().minify_key_prefix
}

fn minify_key_thresh() -> usize {
    I18nAttrs::default().minify_key_thresh
}
