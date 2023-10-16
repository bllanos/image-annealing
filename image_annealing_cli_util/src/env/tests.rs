mod input_environment_variable_error {
    use super::super::EnvironmentVariableNotFoundError;

    #[test]
    fn not_found() {
        let key = "not_found";
        let error = EnvironmentVariableNotFoundError::new(key);
        assert_eq!(
            error.to_string(),
            format!("environment variable \"{}\" not found", key)
        );
        assert_eq!(error.key(), key);
    }
}
