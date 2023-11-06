mod parse_config_file {
    use super::super::parse_config_file;
    use crate::path::TryFromWithPathContext;
    use serde::Deserialize;
    use std::error::Error;
    use std::fmt;
    use std::path::{Path, PathBuf};

    #[derive(Deserialize)]
    struct UnverifiedConfig {
        pub count: usize,
    }

    #[derive(Debug, Eq, PartialEq)]
    struct Config {
        pub count: usize,
        pub path: PathBuf,
    }

    impl TryFromWithPathContext<UnverifiedConfig> for Config {
        type Error = Box<dyn Error>;

        fn try_from_with_path_context<P: AsRef<Path>>(
            value: UnverifiedConfig,
            base_path: P,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                count: value.count,
                path: base_path.as_ref().to_path_buf(),
            })
        }
    }

    #[derive(Debug)]
    struct VerificationFailedError;

    impl VerificationFailedError {
        const VERIFICATION_ERROR_MESSAGE: &'static str = "verification failed";
    }

    impl fmt::Display for VerificationFailedError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", Self::VERIFICATION_ERROR_MESSAGE)
        }
    }

    impl Error for VerificationFailedError {}

    #[derive(Debug)]
    struct FailVerificationConfig;

    impl TryFromWithPathContext<UnverifiedConfig> for FailVerificationConfig {
        type Error = Box<dyn Error>;

        fn try_from_with_path_context<P: AsRef<Path>>(
            _value: UnverifiedConfig,
            _base_path: P,
        ) -> Result<Self, Self::Error> {
            Err(Box::new(VerificationFailedError))
        }
    }

    #[test]
    fn missing_config_file() {
        let path = test_util::path::unverified_absolute_input_path("config/not_found.json");
        test_util::assert_error_contains(
            parse_config_file::<Config, UnverifiedConfig, PathBuf>(path),
            "does not exist",
        );
    }

    #[test]
    fn directory_not_file() {
        let path = test_util::path::unverified_absolute_input_path("config");
        test_util::assert_error_contains(
            parse_config_file::<Config, UnverifiedConfig, PathBuf>(path),
            "is not a file",
        );
    }

    #[test]
    fn malformed_config_file() {
        let path = test_util::path::unverified_absolute_input_path("config/empty.json");
        test_util::assert_error_contains(
            parse_config_file::<Config, UnverifiedConfig, PathBuf>(path),
            "error parsing the contents of",
        );
    }

    #[test]
    fn valid_config_file() -> Result<(), Box<dyn Error>> {
        let path = test_util::path::unverified_absolute_input_path("config/test_stub/valid.json");
        let parent = path.parent().unwrap().to_path_buf();
        let r = parse_config_file::<Config, UnverifiedConfig, PathBuf>(path.clone())?;
        assert_eq!(
            r,
            Config {
                count: 1,
                path: parent
            }
        );
        Ok(())
    }

    #[test]
    fn invalid_config_file() {
        let path = test_util::path::unverified_absolute_input_path("config/test_stub/valid.json");
        test_util::assert_error_contains(
            parse_config_file::<FailVerificationConfig, UnverifiedConfig, PathBuf>(path),
            VerificationFailedError::VERIFICATION_ERROR_MESSAGE,
        );
    }
}
