use std::fmt;
use std::sync::Arc;

use arc_swap::{ArcSwap, Guard};

/// A thread-safe atomically reference-counting string.
pub struct AtomicStr(ArcSwap<String>);

/// A thread-safe view the string that was stored when `AtomicStr::as_str()` was called.
struct GuardedStr(Guard<Arc<String>>);

impl AsRef<str> for GuardedStr {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for GuardedStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

impl AtomicStr {
    /// Create a new `AtomicStr` with the given value.
    pub fn new(value: impl Into<String>) -> Self {
        let arced = Arc::new(value.into());
        Self(ArcSwap::new(arced))
    }

    /// Get the string slice.
    pub fn as_str(&self) -> impl AsRef<str> + fmt::Display {
        GuardedStr(self.0.load())
    }

    /// Get the cloned inner `Arc<String>`.
    pub fn clone_string(&self) -> Arc<String> {
        Guard::into_inner(self.0.load())
    }

    /// Replaces the value at self with src.
    pub fn replace(&self, src: impl Into<String>) {
        let arced = Arc::new(src.into());
        self.0.store(arced);
    }
}

impl<T> From<T> for AtomicStr
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl From<&AtomicStr> for Arc<String> {
    fn from(value: &AtomicStr) -> Self {
        value.clone_string()
    }
}

impl fmt::Display for AtomicStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
