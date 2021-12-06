#[macro_use]
extern crate rust_i18n;

i18n!("./locales");

#[cfg(test)]
mod tests {
    #[test]
    fn it_foo_title() {
        assert_eq!(foo::hello(), "Foo - Hello, World!");
    }

    #[test]
    fn it_t() {
        assert_eq!(t!("hello"), "bar - Hello");
    }
}
