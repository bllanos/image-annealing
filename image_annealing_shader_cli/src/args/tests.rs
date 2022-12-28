mod options {
    use super::super::make_option_parser;

    #[test]
    fn check_options() {
        make_option_parser().check_invariants(true)
    }
}
