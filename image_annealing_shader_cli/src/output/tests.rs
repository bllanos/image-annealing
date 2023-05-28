mod output_config {

    mod with_base_directory {
        use super::super::super::OutputConfig;
        use std::borrow::Cow;
        use std::error::Error;
        use std::path::{Path, PathBuf};

        #[test]
        fn no_directory() -> Result<(), Box<dyn Error>> {
            let config = OutputConfig::with_base_directory::<&Path>(None)?;
            assert_eq!(
                &config,
                &OutputConfig {
                    count_swap: Some(Cow::from(
                        [".", "count_swap.wgsl"].iter().collect::<PathBuf>(),
                    )),
                    create_displacement_goal_default: Some(Cow::from(
                        [".", "create_displacement_goal_default.wgsl"]
                            .iter()
                            .collect::<PathBuf>(),
                    )),
                    create_permutation: Some(Cow::from(
                        [".", "create_permutation.wgsl"].iter().collect::<PathBuf>(),
                    )),
                    permute: Some(Cow::from([".", "permute.wgsl"].iter().collect::<PathBuf>())),
                    swap: Some(Cow::from([".", "swap.wgsl"].iter().collect::<PathBuf>())),
                },
            );
            Ok(())
        }

        #[test]
        fn with_directory() -> Result<(), Box<dyn Error>> {
            let directory = test_util::make_test_output_path(std::iter::empty::<&Path>());
            let config = OutputConfig::with_base_directory::<&Path>(Some(&directory))?;
            assert_eq!(
                &config,
                &OutputConfig {
                    count_swap: Some(Cow::from(directory.join("count_swap.wgsl"))),
                    create_displacement_goal_default: Some(Cow::from(
                        directory.join("create_displacement_goal_default.wgsl")
                    )),
                    create_permutation: Some(Cow::from(directory.join("create_permutation.wgsl"))),
                    permute: Some(Cow::from(directory.join("permute.wgsl"))),
                    swap: Some(Cow::from(directory.join("swap.wgsl"))),
                },
            );
            Ok(())
        }

        #[test]
        fn absent_directory() {
            let directory = test_util::make_test_output_path(["none"]);
            test_util::assert_error_contains(
                OutputConfig::with_base_directory::<&Path>(Some(&directory)),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn file_not_directory() {
            let path = test_util::make_test_data_path(["config", "empty.json"]);
            test_util::assert_error_contains(
                OutputConfig::with_base_directory::<&Path>(Some(&path)),
                "is not a directory", // Note: do not put a platform-dependent path string here
            );
        }
    }
}

mod write_files {
    use super::super::OutputConfig;
    use image_annealing_shader::shader;
    use std::borrow::Cow;
    use std::error::Error;

    #[test]
    fn count_swap_only() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_output_path(["count_swap_only.wgsl"]);
        assert!(!path.is_file());
        let config = OutputConfig {
            count_swap: Some(Cow::from(&path)),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::count_swap(&mut expected)?;
        let actual = std::fs::read(&path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn create_displacement_goal_default_only() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_output_path(["create_displacement_goal_default_only.wgsl"]);
        assert!(!path.is_file());
        let config = OutputConfig {
            create_displacement_goal_default: Some(Cow::from(&path)),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::create_displacement_goal_default(&mut expected)?;
        let actual = std::fs::read(&path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn create_permutation_only() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_output_path(["create_permutation_only.wgsl"]);
        assert!(!path.is_file());
        let config = OutputConfig {
            create_permutation: Some(Cow::from(&path)),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::create_permutation(&mut expected)?;
        let actual = std::fs::read(&path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn permute_only() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_output_path(["permute_only.wgsl"]);
        assert!(!path.is_file());
        let config = OutputConfig {
            permute: Some(Cow::from(&path)),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::permute(&mut expected)?;
        let actual = std::fs::read(&path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn swap_only() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_output_path(["swap_only.wgsl"]);
        assert!(!path.is_file());
        let config = OutputConfig {
            swap: Some(Cow::from(&path)),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        shader::swap(&mut expected)?;
        let actual = std::fs::read(&path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(path)?;

        Ok(())
    }
}

mod write_default_files {
    use image_annealing_shader::shader;
    use std::error::Error;

    #[test]
    fn all_shaders() -> Result<(), Box<dyn Error>> {
        let directory = test_util::make_test_output_path([
            "image_annealing_shader_cli_write_default_files_all_shaders",
        ]);
        assert!(!directory.try_exists()?);
        std::fs::create_dir(&directory)?;

        let count_swap_path = directory.join("count_swap.wgsl");
        assert!(!count_swap_path.is_file());

        let create_displacement_goal_default_path =
            directory.join("create_displacement_goal_default.wgsl");
        assert!(!create_displacement_goal_default_path.is_file());

        let create_permutation_path = directory.join("create_permutation.wgsl");
        assert!(!create_permutation_path.is_file());

        let permute_path = directory.join("permute.wgsl");
        assert!(!permute_path.is_file());

        let swap_path = directory.join("swap.wgsl");
        assert!(!swap_path.is_file());

        super::super::write_default_files(Some(&directory))?;

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

        std::fs::remove_dir(directory)?;
        Ok(())
    }
}
