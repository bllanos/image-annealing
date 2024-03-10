mod options {
    use super::super::make_option_parser;
    use test_util::{side_effect, TestResult};

    #[test]
    fn check_options() {
        make_option_parser().check_invariants(true)
    }

    #[test]
    fn check_that_markdown_documentation_is_current() -> TestResult {
        let option_parser = make_option_parser();
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        let markdown = option_parser.render_markdown(app_name);

        let mut documentation_path = test_util::repository::make_documentation_path();
        documentation_path.push(app_name);

        test_util::repository::check_that_file_is_current_and_create_new_file(
            side_effect::open_file,
            side_effect::create_file,
            documentation_path,
            "md",
            &markdown,
        )
    }
}
