mod parse_config_file {
    use super::super::super::{Config, UnverifiedConfig, UnverifiedCreateDisplacementGoalConfig};
    use super::super::parse_config_file;
    use image_annealing_cli_util::path::TryIntoWithPathContext;
    use std::error::Error;

    #[test]
    fn missing_config_file() {
        let path = test_util::path::unverified_absolute_input_path("config/not_found.json");
        test_util::assert_error_contains(parse_config_file(path), "does not exist");
    }

    #[test]
    fn directory_not_file() {
        let path = test_util::path::absolute_input_directory("config");
        test_util::assert_error_contains(parse_config_file(path.0), "is not a file");
    }

    #[test]
    fn malformed_config_file() {
        let path = test_util::path::absolute_input_file("config/empty.json");
        test_util::assert_error_contains(
            parse_config_file(path.0),
            "configuration file deserialization error",
        );
    }

    #[test]
    fn valid_create_displacement_goal_config_file() -> Result<(), Box<dyn Error>> {
        let path = test_util::path::absolute_input_file(
            "config/shader/create_displacement_goal/copy_image.json",
        );
        let r = parse_config_file(path.0)?;

        let shader_body_path =
            test_util::path::relative_input_file("shader/create_displacement_goal/copy_image.wgsl");
        let config: Config =
            (UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
                body: shader_body_path,
            }))
            .try_into_with_path_context(test_util::path::base_input().0)?;

        assert_eq!(r, config);
        Ok(())
    }

    #[test]
    fn invalid_create_displacement_goal_config_file() {
        let path = test_util::path::absolute_input_file(
            "config/shader/create_displacement_goal/invalid.json",
        );
        test_util::assert_error_contains(parse_config_file(path.0), "does not exist");
    }
}
