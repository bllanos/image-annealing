mod output_config {

    mod with_base_directory {
        use super::super::super::OutputConfig;
        use image_annealing_cli_util::path::TryIntoWithPathContext;
        use std::error::Error;
        use std::path::Path;

        #[test]
        fn no_directory() -> Result<(), Box<dyn Error>> {
            let config = OutputConfig::with_base_directory::<&Path>(None)?;
            let base_path = Path::new(".");
            assert_eq!(
                &config,
                &OutputConfig {
                    count_swap: Some(
                        test_util::path::relative_output_file("count_swap.wgsl")
                            .try_into_with_path_context(base_path)?
                    ),
                    create_displacement_goal_default: Some(
                        test_util::path::relative_output_file(
                            "create_displacement_goal_default.wgsl"
                        )
                        .try_into_with_path_context(base_path)?
                    ),
                    create_permutation: Some(
                        test_util::path::relative_output_file("create_permutation.wgsl")
                            .try_into_with_path_context(base_path)?
                    ),
                    permute: Some(
                        test_util::path::relative_output_file("permute.wgsl")
                            .try_into_with_path_context(base_path)?
                    ),
                    swap: Some(
                        test_util::path::relative_output_file("swap.wgsl")
                            .try_into_with_path_context(base_path)?
                    ),
                },
            );
            Ok(())
        }

        #[test]
        fn with_directory() -> Result<(), Box<dyn Error>> {
            let directory = test_util::path::base_output().0;
            let config = OutputConfig::with_base_directory(Some(&directory))?;
            assert_eq!(
                &config,
                &OutputConfig {
                    count_swap: Some(test_util::path::absolute_output_file("count_swap.wgsl")),
                    create_displacement_goal_default: Some(test_util::path::absolute_output_file(
                        "create_displacement_goal_default.wgsl"
                    )),
                    create_permutation: Some(test_util::path::absolute_output_file(
                        "create_permutation.wgsl"
                    )),
                    permute: Some(test_util::path::absolute_output_file("permute.wgsl")),
                    swap: Some(test_util::path::absolute_output_file("swap.wgsl")),
                },
            );
            Ok(())
        }

        #[test]
        fn absent_directory() {
            let directory = test_util::path::absolute_output_directory("not_found");
            test_util::assert_error_contains(
                OutputConfig::with_base_directory(Some(&directory.0)),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn file_not_directory() {
            let path = test_util::path::absolute_input_file("config/empty.json");
            test_util::assert_error_contains(
                OutputConfig::with_base_directory(Some(&path.0)),
                "is not a directory", // Note: do not put a platform-dependent path string here
            );
        }
    }
}

mod write_files {
    use super::super::OutputConfig;
    use image_annealing_shader::shader;
    use std::error::Error;

    #[test]
    fn count_swap_only() -> Result<(), Box<dyn Error>> {
        let path = test_util::path::absolute_output_file("count_swap_only.wgsl");
        assert!(!path.0.is_file());
        let config = OutputConfig {
            count_swap: Some(path.clone()),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::count_swap(&mut expected)?;
        let actual = std::fs::read(&path.0)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(&path.0)?;

        Ok(())
    }

    #[test]
    fn create_displacement_goal_default_only() -> Result<(), Box<dyn Error>> {
        let path =
            test_util::path::absolute_output_file("create_displacement_goal_default_only.wgsl");
        assert!(!path.0.is_file());
        let config = OutputConfig {
            create_displacement_goal_default: Some(path.clone()),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::create_displacement_goal_default(&mut expected)?;
        let actual = std::fs::read(&path.0)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(&path.0)?;

        Ok(())
    }

    #[test]
    fn create_permutation_only() -> Result<(), Box<dyn Error>> {
        let path = test_util::path::absolute_output_file("create_permutation_only.wgsl");
        assert!(!path.0.is_file());
        let config = OutputConfig {
            create_permutation: Some(path.clone()),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::create_permutation(&mut expected)?;
        let actual = std::fs::read(&path.0)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(&path.0)?;

        Ok(())
    }

    #[test]
    fn permute_only() -> Result<(), Box<dyn Error>> {
        let path = test_util::path::absolute_output_file("permute_only.wgsl");
        assert!(!path.0.is_file());
        let config = OutputConfig {
            permute: Some(path.clone()),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::permute(&mut expected)?;
        let actual = std::fs::read(&path.0)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(&path.0)?;

        Ok(())
    }

    #[test]
    fn swap_only() -> Result<(), Box<dyn Error>> {
        let path = test_util::path::absolute_output_file("swap_only.wgsl");
        assert!(!path.0.is_file());
        let config = OutputConfig {
            swap: Some(path.clone()),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::swap(&mut expected)?;
        let actual = std::fs::read(&path.0)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(&path.0)?;

        Ok(())
    }
}

mod write_default_files {
    use image_annealing_shader::shader;
    use std::error::Error;

    #[test]
    fn all_shaders() -> Result<(), Box<dyn Error>> {
        let directory = test_util::unique_absolute_output_directory!();
        assert!(!directory.0.try_exists()?);
        std::fs::create_dir(&directory.0)?;

        let count_swap_path = directory.0.join("count_swap.wgsl");
        assert!(!count_swap_path.is_file());

        let create_displacement_goal_default_path =
            directory.0.join("create_displacement_goal_default.wgsl");
        assert!(!create_displacement_goal_default_path.is_file());

        let create_permutation_path = directory.0.join("create_permutation.wgsl");
        assert!(!create_permutation_path.is_file());

        let permute_path = directory.0.join("permute.wgsl");
        assert!(!permute_path.is_file());

        let swap_path = directory.0.join("swap.wgsl");
        assert!(!swap_path.is_file());

        super::super::write_default_files(Some(&directory.0))?;

        let mut expected: Vec<u8> = Vec::new();

        shader::count_swap(&mut expected)?;
        let mut actual = std::fs::read(&count_swap_path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(count_swap_path)?;

        expected.clear();
        shader::create_displacement_goal_default(&mut expected)?;
        actual = std::fs::read(&create_displacement_goal_default_path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(create_displacement_goal_default_path)?;

        expected.clear();
        shader::create_permutation(&mut expected)?;
        actual = std::fs::read(&create_permutation_path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(create_permutation_path)?;

        expected.clear();
        shader::permute(&mut expected)?;
        actual = std::fs::read(&permute_path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(permute_path)?;

        expected.clear();
        shader::swap(&mut expected)?;
        actual = std::fs::read(&swap_path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(swap_path)?;

        std::fs::remove_dir(&directory.0)?;
        Ok(())
    }
}
