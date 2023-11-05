mod config_try_from_unverified_config {
    mod create_displacement_goal {
        use super::super::super::{
            Config, UnverifiedConfig, UnverifiedCreateDisplacementGoalConfig,
        };
        use image_annealing_cli_util::path::{
            InputFilePath, TryFromWithPathContext, TryIntoWithPathContext,
        };
        use image_annealing_shader::CreateDisplacementGoalShaderContent;
        use std::borrow::Cow;
        use std::error::Error;
        use std::fs;

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            let shader_body_path = test_util::path::relative_input_file(
                "shader/create_displacement_goal/copy_image.wgsl",
            );
            let unverified_config =
                UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
                    body: shader_body_path.clone(),
                });

            let r: Config =
                unverified_config.try_into_with_path_context(test_util::path::base_input().0)?;
            assert_eq!(
                r,
                Config::CreateDisplacementGoal(CreateDisplacementGoalShaderContent {
                    body: Cow::Owned(fs::read_to_string(
                        InputFilePath::try_from_with_path_context(
                            shader_body_path,
                            test_util::path::base_input().0
                        )?
                        .0
                    )?),
                })
            );
            Ok(())
        }

        #[test]
        fn missing_body() {
            let shader_body_path = test_util::path::relative_input_file(
                "shader/create_displacement_goal/not_found.wgsl",
            );
            let unverified_config =
                UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
                    body: shader_body_path,
                });

            let r =
                <Config as TryFromWithPathContext<UnverifiedConfig>>::try_from_with_path_context(
                    unverified_config,
                    test_util::path::base_input().0,
                );
            test_util::assert_error_contains(r, "does not exist");
        }
    }
}
