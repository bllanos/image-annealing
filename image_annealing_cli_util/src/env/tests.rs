mod environment_variable_not_unicode_error {
    use super::super::EnvironmentVariableNotUnicodeError;

    #[test]
    fn check_display_string() {
        let key = "TEST_KEY";
        let value = "Invalid";
        assert_eq!(
            EnvironmentVariableNotUnicodeError::new(key, value.into()).to_string(),
            format!(
                "environment variable \"{key}\" has a value that is invalid UTF-8, \"{value}\""
            )
        );
    }
}

mod parse_environment_variables {
    use super::super::{EnvironmentVariableData, EnvironmentVariableNotUnicodeError};
    use std::ffi::{OsStr, OsString};

    #[test]
    fn empty_value() {
        let key = OsStr::new("TEST_KEY");
        assert_eq!(
            super::super::parse_environment_variables([EnvironmentVariableData {
                key,
                value: None
            }]),
            Ok([None])
        );
    }

    #[test]
    fn valid_utf8() {
        let key = OsStr::new("TEST_KEY");
        let value = "value";
        assert_eq!(
            super::super::parse_environment_variables([EnvironmentVariableData {
                key,
                value: Some(value.into())
            }]),
            Ok([Some(value.into())])
        );
    }

    #[test]
    fn invalid_utf8() {
        let key = OsStr::new("TEST_KEY");
        // Reference: <https://doc.rust-lang.org/std/str/struct.Utf8Error.html>
        let value = unsafe { OsString::from_encoded_bytes_unchecked(vec![0, 159, 146, 150]) };
        assert_eq!(
            super::super::parse_environment_variables([EnvironmentVariableData {
                key,
                value: Some(value.clone())
            }]),
            Err(EnvironmentVariableNotUnicodeError {
                key: key.into(),
                value,
            })
        );
    }
}

mod invalid_boolean_environment_variable_value_error {
    use super::super::InvalidBooleanEnvironmentVariableValueError;
    #[test]
    fn check_display_string() {
        let value = "Invalid";
        assert_eq!(
            InvalidBooleanEnvironmentVariableValueError(value.into()).to_string(),
            format!(
                "Boolean environment variable value, \"{value}\", is not a case-insensitive match with \"true\", \"1\", \"false\" or \"0\""
            )
        );
    }
}

mod parse_bool_from_environment_variable {
    use super::super::InvalidBooleanEnvironmentVariableValueError;

    #[test]
    fn parse_from_all_valid_strings() {
        for (string, value) in
            [("true", true), ("1", true), ("false", false), ("0", false)].into_iter()
        {
            assert_eq!(
                super::super::parse_bool_from_environment_variable(string),
                Ok(value)
            );
        }
    }

    #[test]
    fn parsing_from_a_string_is_case_insensitive() {
        for (string, value) in [("TRUE", true), ("FALSE", false)].into_iter() {
            assert_eq!(
                super::super::parse_bool_from_environment_variable(string),
                Ok(value)
            );
        }
    }

    #[test]
    fn parsing_from_a_string_ignores_padding_whitespace() {
        assert_eq!(
            super::super::parse_bool_from_environment_variable(" true "),
            Ok(true)
        );
    }

    #[test]
    fn an_invalid_string_results_in_a_parsing_error() {
        let invalid_input = "Invalid";
        let error = super::super::parse_bool_from_environment_variable(invalid_input).unwrap_err();
        assert_eq!(
            error,
            InvalidBooleanEnvironmentVariableValueError(invalid_input.to_string())
        );
    }
}

mod filepath_environment_variable_value {
    use super::super::FilepathEnvironmentVariableValue;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn strings_are_parsed_as_is() {
        let input = "/path/ 1 \\2/3";
        assert_eq!(
            FilepathEnvironmentVariableValue::from(OsString::from(input)),
            FilepathEnvironmentVariableValue(PathBuf::from(input))
        );
    }
}
