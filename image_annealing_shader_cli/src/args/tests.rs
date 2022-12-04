mod options {
    use super::super::{make_option_parser, Options};
    use bpaf::ParseFailure;

    #[test]
    fn check_options() {
        make_option_parser().check_invariants(true)
    }

    #[test]
    fn no_output_directory() -> Result<(), ParseFailure> {
        let v = &[];
        let options = make_option_parser().run_inner(v.into())?;
        assert_eq!(
            options,
            Options {
                output_directory: None
            }
        );
        Ok(())
    }

    #[test]
    fn with_output_directory() -> Result<(), ParseFailure> {
        let path = test_utils::make_test_output_path_string(std::iter::empty::<&str>());
        let v = &["-d", &path];
        let options = make_option_parser().run_inner(v.into())?;
        assert_eq!(
            options,
            Options {
                output_directory: Some(path.into())
            }
        );
        Ok(())
    }

    #[test]
    fn additional_args() {
        let path = test_utils::make_test_output_path_string(std::iter::empty::<&str>());
        let v = &["-d", &path, "other_arg"];
        let message = make_option_parser()
            .run_inner(v.into())
            .unwrap_err()
            .unwrap_stderr();
        test_utils::assert_error_contains::<(), String>(
            Err(message),
            "No such command: `other_arg`",
        );
    }
}
