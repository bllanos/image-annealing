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
            let directory = test_utils::make_test_output_path(&[] as &[&Path]);
            let config = OutputConfig::with_base_directory::<&Path>(Some(&directory))?;
            assert_eq!(
                &config,
                &OutputConfig {
                    count_swap: Some(Cow::from(directory.join("count_swap.wgsl"))),
                    create_permutation: Some(Cow::from(directory.join("create_permutation.wgsl"))),
                    permute: Some(Cow::from(directory.join("permute.wgsl"))),
                    swap: Some(Cow::from(directory.join("swap.wgsl"))),
                },
            );
            Ok(())
        }

        #[test]
        fn absent_directory() {
            let directory = test_utils::make_test_output_path(&["none"]);
            test_utils::assert_error_contains(
                OutputConfig::with_base_directory::<&Path>(Some(&directory)),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn file_not_directory() {
            let path = test_utils::make_test_data_path(&["config", "empty.json"]);
            test_utils::assert_error_contains(
                OutputConfig::with_base_directory::<&Path>(Some(&path)),
                "is not a directory", // Note: do not put a platform-dependent path string here
            );
        }
    }
}

mod write_files {
    use super::super::OutputConfig;
    use std::borrow::Cow;
    use std::error::Error;

    #[test]
    fn count_swap_only() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_output_path(&["count_swap_only.wgsl"]);
        if path.is_file() {
            panic!("Output shader file already exists in the filesystem.")
        }
        let config = OutputConfig {
            count_swap: Some(Cow::from(&path)),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        crate::shader::count_swap(&mut expected)?;
        let actual = std::fs::read(&path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn create_permutation_only() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_output_path(&["create_permutation_only.wgsl"]);
        if path.is_file() {
            panic!("Output shader file already exists in the filesystem.")
        }
        let config = OutputConfig {
            create_permutation: Some(Cow::from(&path)),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        crate::shader::create_permutation(&mut expected)?;
        let actual = std::fs::read(&path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn permute_only() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_output_path(&["permute_only.wgsl"]);
        if path.is_file() {
            panic!("Output shader file already exists in the filesystem.")
        }
        let config = OutputConfig {
            permute: Some(Cow::from(&path)),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        crate::shader::permute(&mut expected)?;
        let actual = std::fs::read(&path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn swap_only() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_output_path(&["swap_only.wgsl"]);
        if path.is_file() {
            panic!("Output shader file already exists in the filesystem.")
        }
        let config = OutputConfig {
            swap: Some(Cow::from(&path)),
            ..Default::default()
        };
        super::super::write_files(&config)?;
        let mut expected: Vec<u8> = Vec::new();
        crate::shader::swap(&mut expected)?;
        let actual = std::fs::read(&path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(path)?;

        Ok(())
    }
}

mod write_default_files {
    use std::error::Error;
    use std::path::Path;

    #[test]
    fn all_shaders() -> Result<(), Box<dyn Error>> {
        let directory = test_utils::make_test_output_path(&[] as &[&Path]);

        let count_swap_path = test_utils::make_test_output_path(&["count_swap.wgsl"]);
        if count_swap_path.is_file() {
            panic!("count_swap shader file already exists in the filesystem.")
        }

        let create_permutation_path =
            test_utils::make_test_output_path(&["create_permutation.wgsl"]);
        if create_permutation_path.is_file() {
            panic!("create_permutation shader file already exists in the filesystem.")
        }

        let permute_path = test_utils::make_test_output_path(&["permute.wgsl"]);
        if permute_path.is_file() {
            panic!("permute shader file already exists in the filesystem.")
        }

        let swap_path = test_utils::make_test_output_path(&["swap.wgsl"]);
        if swap_path.is_file() {
            panic!("swap shader file already exists in the filesystem.")
        }

        super::super::write_default_files(Some(directory))?;

        let mut expected: Vec<u8> = Vec::new();

        crate::shader::count_swap(&mut expected)?;
        let mut actual = std::fs::read(&count_swap_path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(count_swap_path)?;

        expected.clear();
        crate::shader::create_permutation(&mut expected)?;
        actual = std::fs::read(&create_permutation_path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(create_permutation_path)?;

        expected.clear();
        crate::shader::permute(&mut expected)?;
        actual = std::fs::read(&permute_path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(permute_path)?;

        expected.clear();
        crate::shader::swap(&mut expected)?;
        actual = std::fs::read(&swap_path)?;
        assert_eq!(actual, expected);
        std::fs::remove_file(swap_path)?;

        Ok(())
    }
}
