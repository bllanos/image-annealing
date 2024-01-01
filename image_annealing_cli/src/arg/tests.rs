mod options {
    use super::super::make_option_parser;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::PathBuf;

    #[test]
    fn check_options() {
        make_option_parser().check_invariants(true)
    }

    #[test]
    fn check_that_markdown_documentation_is_current() {
        let option_parser = make_option_parser();
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        let markdown = option_parser.render_markdown(app_name);

        let mut documentation_path = PathBuf::new();
        documentation_path.push("..");
        documentation_path.push("docs");
        documentation_path.push(app_name);
        documentation_path.set_extension("md");

        let mut reference_file = match File::open(&documentation_path) {
            Err(why) => panic!(
                "failed to open documentation file {}: {}",
                documentation_path.display(),
                why
            ),
            Ok(file) => file,
        };

        let mut reference_markdown = String::new();
        if let Err(why) = reference_file.read_to_string(&mut reference_markdown) {
            panic!(
                "failed to read from documentation file {}: {}",
                documentation_path.display(),
                why
            );
        }

        if reference_markdown != markdown {
            let updated_file_path = documentation_path.as_path().with_extension("md.new");

            let mut updated_file = match File::create(&updated_file_path) {
                Err(why) => panic!(
                    "failed to open an updated documentation file {}: {}",
                    updated_file_path.display(),
                    why
                ),
                Ok(file) => file,
            };

            if let Err(why) = updated_file.write_all(markdown.as_bytes()) {
                panic!(
                    "failed to write to updated documentation file {}: {}",
                    updated_file_path.display(),
                    why
                );
            }
            panic!("{} is out of date and an updated version was saved, {}, to be used to overwrite it", documentation_path.display(), updated_file_path.display());
        }
    }
}
