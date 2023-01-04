#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("../locales");

fn get_text() -> String {
    t!("hello")
}

fn main() {
    println!("{}, Longbridge!", t!("hello"));

    //
    println!("{}", example_base::hello("Longbridge"));

    rust_i18n::set_locale("fr");
    println!("{}, Longbridge!", t!("hello"));
    println!("{}", example_base::hello("Longbridge"));
}

#[cfg(test)]
mod tests {
    use crate::get_text;

    #[test]
    fn test_get_text() {
        rust_i18n::set_locale("en");
        assert_eq!("Hello", get_text());
        rust_i18n::set_locale("fr");
        assert_eq!("Bonjour", get_text());
    }

    #[test]
    fn test_example_hello() {
        rust_i18n::set_locale("en");
        assert_eq!("Hello example-base: Jason", example_base::hello("Jason"));
        rust_i18n::set_locale("fr");
        assert_eq!("Bonjour example-base: Jason", example_base::hello("Jason"));
    }
}
