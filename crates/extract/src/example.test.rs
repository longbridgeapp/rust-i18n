mod Example {
    fn hello() {
        // comment 1
        let x = t!("hello");
        let x = t!("views.message.title", locale = "en", name = "Jason");
        // comment 3
        let x = t!("views.message.description", name = "Jason");

        // comment 4
        {
            t!(r##"Use YAML for mapping localized text, 
            and support mutiple YAML files merging."##);

            t!(r##"Use YAML for mapping localized text,
and support mutiple YAML files merging."##);
        }

        t!("The table below describes some of those behaviours.");
        // Will remove spaces for avoid duplication.
        t!("The table     below describes some     of those behaviours.");
    }
}
