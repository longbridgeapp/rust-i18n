use std::ops::Add;
use std::thread::spawn;
use std::time::{Duration, Instant};

use rust_i18n::{set_locale, t};

rust_i18n::i18n!("locales", fallback = "en");

#[test]
fn test_load_and_store() {
    let end = Instant::now().add(Duration::from_secs(3));
    let store = spawn(move || {
        let mut i = 0u32;
        while Instant::now() < end {
            for _ in 0..100 {
                i = i.wrapping_add(1);
                if i % 2 == 0 {
                    set_locale(&format!("en-{i}"));
                } else {
                    set_locale(&format!("fr-{i}"));
                }
            }
        }
    });
    let load = spawn(move || {
        while Instant::now() < end {
            for _ in 0..100 {
                t!("hello");
            }
        }
    });
    store.join().unwrap();
    load.join().unwrap();
}
