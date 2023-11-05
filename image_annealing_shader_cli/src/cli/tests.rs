mod assemble_shader {
    use super::super::assemble_shader;
    use crate::args::AssembleShaderOptions;
    use crate::config::{Config, UnverifiedConfig, UnverifiedCreateDisplacementGoalConfig};
    use image_annealing_cli_util::path::TryIntoWithPathContext;
    use image_annealing_cli_util::text::UnverifiedInputTextFilePath;
    use image_annealing_shader::shader;
    use std::error::Error;

    #[test]
    fn valid_create_displacement_goal_shader() -> Result<(), Box<dyn Error>> {
        let output_file = test_util::unique_absolute_output_file!();
        assert!(!output_file.0.is_file());

        let shader_body_path = UnverifiedInputTextFilePath(test_util::path::relative_input_file(
            "shader/create_displacement_goal/copy_image.wgsl",
        ));

        let config: Config =
            (UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
                body: shader_body_path,
            }))
            .try_into_with_path_context(test_util::path::base_input().0)?;
        assemble_shader(&AssembleShaderOptions {
            config: config.clone(),
            output_file: output_file.0.clone().into_owned(),
        })?;

        let mut expected: Vec<u8> = Vec::new();
        match config {
            Config::CreateDisplacementGoal(ref inner) => {
                shader::create_displacement_goal_custom(&mut expected, inner)?
            }
        }
        let actual = std::fs::read(&output_file.0)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(&output_file.0)?;

        Ok(())
    }

    #[test]
    fn missing_output_directory() -> Result<(), Box<dyn Error>> {
        let output_file =
            test_util::path::unverified_absolute_output_path("not_found/cannot_create");
        let shader_body_path = UnverifiedInputTextFilePath(test_util::path::relative_input_file(
            "shader/create_displacement_goal/copy_image.wgsl",
        ));

        test_util::assert_error_contains(
            assemble_shader(&AssembleShaderOptions {
                config: (UnverifiedConfig::CreateDisplacementGoal(
                    UnverifiedCreateDisplacementGoalConfig {
                        body: shader_body_path,
                    },
                ))
                .try_into_with_path_context(test_util::path::base_input().0)?,
                output_file,
            }),
            "No such file or directory",
        );
        Ok(())
    }

    #[test]
    fn parse_error() -> Result<(), Box<dyn Error>> {
        let output_file = test_util::unique_absolute_output_file!();
        assert!(!output_file.0.is_file());

        let shader_body_path = UnverifiedInputTextFilePath(test_util::path::relative_input_file(
            "shader/create_displacement_goal/parse_error.wgsl",
        ));

        let config: Config =
            (UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
                body: shader_body_path,
            }))
            .try_into_with_path_context(test_util::path::base_input().0)?;
        test_util::assert_error_contains(
            assemble_shader(&AssembleShaderOptions {
                config: config.clone(),
                output_file: output_file.0.clone().into_owned(),
            }),
            "generated shader is not valid WGSL",
        );

        let mut expected: Vec<u8> = Vec::new();
        match config {
            Config::CreateDisplacementGoal(ref inner) => {
                shader::create_displacement_goal_custom(&mut expected, inner)?
            }
        }
        let actual = std::fs::read(&output_file.0)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(&output_file.0)?;

        Ok(())
    }

    #[test]
    fn module_error() -> Result<(), Box<dyn Error>> {
        let output_file = test_util::unique_absolute_output_file!();
        assert!(!output_file.0.is_file());

        let shader_body_path = UnverifiedInputTextFilePath(test_util::path::relative_input_file(
            "shader/create_displacement_goal/module_error.wgsl",
        ));

        let config: Config =
            (UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
                body: shader_body_path,
            }))
            .try_into_with_path_context(test_util::path::base_input().0)?;
        test_util::assert_error_contains(
            assemble_shader(&AssembleShaderOptions {
                config: config.clone(),
                output_file: output_file.0.clone().into_owned(),
            }),
            "generated shader module validation failed",
        );

        let mut expected: Vec<u8> = Vec::new();
        match config {
            Config::CreateDisplacementGoal(ref inner) => {
                shader::create_displacement_goal_custom(&mut expected, inner)?
            }
        }
        let actual = std::fs::read(&output_file.0)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(&output_file.0)?;

        Ok(())
    }
}
