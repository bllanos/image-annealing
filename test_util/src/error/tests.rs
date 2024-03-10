mod test_error {
    use super::super::TestError;
    #[test]
    fn check_display_string() {
        let value = "error message";
        assert_eq!(TestError(value.into()).to_string(), value);
    }
}

mod test_result_equals_string {
    #[test]
    fn ok_does_not_equal_a_string() {
        assert!(!super::super::test_result_equals_string(Ok(()), "string"));
    }
}
