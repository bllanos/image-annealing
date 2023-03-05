mod assemble_shader {
    use super::super::assemble_shader;
    use crate::args::AssembleShaderOptions;
    use crate::config::{Config, UnverifiedConfig, UnverifiedCreateDisplacementGoalConfig};
    use image_annealing_shader::shader;
    use std::error::Error;

    #[test]
    fn valid_create_displacement_goal_shader() -> Result<(), Box<dyn Error>> {
        let output_file =
            test_util::make_test_output_path(["cli_assemble_shader_create_displacement_goal.wgsl"]);
        assert!(!output_file.is_file());

        let shader_body_path = test_util::make_test_data_path_string([
            "shader",
            "create_displacement_goal",
            "copy_image.wgsl",
        ]);

        let config: Config =
            (UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
                body: shader_body_path,
            }))
            .try_into()?;
        assemble_shader(&AssembleShaderOptions {
            config: config.clone(),
            output_file: output_file.clone(),
        })?;

        let mut expected: Vec<u8> = Vec::new();
        match config {
            Config::CreateDisplacementGoal(ref inner) => {
                shader::create_displacement_goal_custom(&mut expected, inner)?
            }
        }
        let actual = std::fs::read(&output_file)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(output_file)?;

        Ok(())
    }

    #[test]
    fn missing_output_directory() -> Result<(), Box<dyn Error>> {
        let output_file = test_util::make_test_output_path(["not_found", "cannot_create.wgsl"]);
        let shader_body_path = test_util::make_test_data_path_string([
            "shader",
            "create_displacement_goal",
            "copy_image.wgsl",
        ]);

        test_util::assert_error_contains(
            assemble_shader(&AssembleShaderOptions {
                config: (UnverifiedConfig::CreateDisplacementGoal(
                    UnverifiedCreateDisplacementGoalConfig {
                        body: shader_body_path,
                    },
                ))
                .try_into()
                .unwrap(),
                output_file,
            }),
            "No such file or directory",
        );
        Ok(())
    }

    #[test]
    fn parse_error() -> Result<(), Box<dyn Error>> {
        let output_file = test_util::make_test_output_path([
            "cli_assemble_shader_create_displacement_goal_parse_error.wgsl",
        ]);
        assert!(!output_file.is_file());

        let shader_body_path = test_util::make_test_data_path_string([
            "shader",
            "create_displacement_goal",
            "parse_error.wgsl",
        ]);

        let config: Config =
            (UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
                body: shader_body_path,
            }))
            .try_into()?;
        test_util::assert_error_contains(
            assemble_shader(&AssembleShaderOptions {
                config: config.clone(),
                output_file: output_file.clone(),
            }),
            "generated shader is not valid WGSL",
        );

        let mut expected: Vec<u8> = Vec::new();
        match config {
            Config::CreateDisplacementGoal(ref inner) => {
                shader::create_displacement_goal_custom(&mut expected, inner)?
            }
        }
        let actual = std::fs::read(&output_file)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(output_file)?;

        Ok(())
    }

    #[test]
    fn module_error() -> Result<(), Box<dyn Error>> {
        let output_file = test_util::make_test_output_path([
            "cli_assemble_shader_create_displacement_goal_module_error.wgsl",
        ]);
        assert!(!output_file.is_file());

        let shader_body_path = test_util::make_test_data_path_string([
            "shader",
            "create_displacement_goal",
            "module_error.wgsl",
        ]);

        let config: Config =
            (UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
                body: shader_body_path,
            }))
            .try_into()?;
        test_util::assert_error_contains(
            assemble_shader(&AssembleShaderOptions {
                config: config.clone(),
                output_file: output_file.clone(),
            }),
            "generated shader module validation failed",
        );

        let mut expected: Vec<u8> = Vec::new();
        match config {
            Config::CreateDisplacementGoal(ref inner) => {
                shader::create_displacement_goal_custom(&mut expected, inner)?
            }
        }
        let actual = std::fs::read(&output_file)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(output_file)?;

        Ok(())
    }
}
