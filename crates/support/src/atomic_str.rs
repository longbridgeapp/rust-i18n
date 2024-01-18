use std::fmt;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

/// A thread-safe atomically reference-counting string.
pub struct AtomicStr(AtomicPtr<String>);

impl AtomicStr {
    /// Create a new `AtomicStr` with the given value.
    pub fn new(value: impl Into<String>) -> Self {
        let arced = Arc::new(value.into());
        Self(AtomicPtr::new(Arc::into_raw(arced) as _))
    }

    /// Get the string slice.
    pub fn as_str(&self) -> &str {
        unsafe {
            let arced_ptr = self.0.load(Ordering::SeqCst);
            assert!(!arced_ptr.is_null());
            &*arced_ptr
        }
    }

    /// Get the cloned inner `Arc<String>`.
    pub fn clone_string(&self) -> Arc<String> {
        unsafe {
            let arced_ptr = self.0.load(Ordering::SeqCst);
            assert!(!arced_ptr.is_null());
            Arc::increment_strong_count(arced_ptr);
            Arc::from_raw(arced_ptr)
        }
    }

    /// Replaces the value at self with src, returning the old value, without dropping either.
    pub fn replace(&self, src: impl Into<String>) -> Arc<String> {
        unsafe {
            let arced_new = Arc::new(src.into());
            let arced_old_ptr = self.0.swap(Arc::into_raw(arced_new) as _, Ordering::SeqCst);
            assert!(!arced_old_ptr.is_null());
            Arc::from_raw(arced_old_ptr)
        }
    }
}

impl Drop for AtomicStr {
    fn drop(&mut self) {
        unsafe {
            let arced_ptr = self.0.swap(std::ptr::null_mut(), Ordering::SeqCst);
            assert!(!arced_ptr.is_null());
            let _ = Arc::from_raw(arced_ptr);
        }
    }
}

impl AsRef<str> for AtomicStr {
    fn as_ref(&self) -> &str {
        self.as_str()
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
