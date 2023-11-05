mod input_environment_variable_error {
    use super::super::EnvironmentVariableAccessError;

    #[test]
    fn not_found() {
        let key = "not_found";
        let error = EnvironmentVariableAccessError::new(key);
        assert_eq!(
            error.to_string(),
            format!(
                "environment variable \"{}\" was not found or could not be accessed",
                key
            )
        );
        assert_eq!(error.key(), key);
    }
}

mod var_os {
    use super::super::EnvironmentVariableAccessError;
    use std::env;
    use std::error::Error;

    #[test]
    fn exists() -> Result<(), Box<dyn Error>> {
        let key = format!("{}.{}.{}", module_path!(), line!(), column!());
        let value = "environment_variable_value";
        env::set_var(&key, value);
        assert_eq!(super::super::var_os(&key)?, value);
        env::remove_var(&key);
        Ok(())
    }

    #[test]
    fn not_found() {
        let key = format!("{}.{}.{}", module_path!(), line!(), column!());
        assert_eq!(
            super::super::var_os(&key),
            Err(EnvironmentVariableAccessError::new(&key)),
        );
    }
}
