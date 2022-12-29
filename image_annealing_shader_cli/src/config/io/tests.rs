mod parse_config_file {
    use super::super::super::{Config, UnverifiedConfig};
    use super::super::parse_config_file;
    use std::error::Error;

    #[test]
    fn missing_config_file() {
        let path = test_util::make_test_data_path(["config", "not_found.json"]);
        test_util::assert_error_contains(parse_config_file(path), "does not exist");
    }

    #[test]
    fn directory_not_file() {
        let path = test_util::make_test_data_path(["config"]);
        test_util::assert_error_contains(parse_config_file(path), "is not a file");
    }

    #[test]
    fn malformed_config_file() {
        let path = test_util::make_test_data_path(["config", "empty.json"]);
        test_util::assert_error_contains(
            parse_config_file(path),
            "configuration file deserialization error",
        );
    }

    #[test]
    fn valid_create_displacement_goal_config_file() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_data_path([
            "config",
            "shader",
            "create_displacement_goal",
            "copy_image.json",
        ]);
        let r = parse_config_file(path)?;

        let shader_body_path = test_util::make_test_data_path_string([
            "shader",
            "create_displacement_goal",
            "copy_image.wgsl",
        ]);
        let config: Config = (UnverifiedConfig::CreateDisplacementGoal {
            body: shader_body_path,
        })
        .try_into()?;

        assert_eq!(r, config);
        Ok(())
    }

    #[test]
    fn invalid_create_displacement_goal_config_file() {
        let path = test_util::make_test_data_path([
            "config",
            "shader",
            "create_displacement_goal",
            "invalid.json",
        ]);
        test_util::assert_error_contains(parse_config_file(path), "does not exist");
    }
}
