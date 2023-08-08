use image_annealing_cli_util::path::{
    InputDirectoryPath, InputFilePath, OutputDirectoryPath, OutputFilePath, TryFromWithPathContext,
    UnverifiedInputDirectoryPath, UnverifiedInputFilePath, UnverifiedOutputDirectoryPath,
    UnverifiedOutputFilePath,
};
use relative_path::RelativePath;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

const TEST_UTIL_BASE_INPUT_PATH: &'static str = "TEST_UTIL_BASE_INPUT_PATH";

const TEST_UTIL_BASE_OUTPUT_PATH: &'static str = "TEST_UTIL_BASE_OUTPUT_PATH";

const CARGO_MANIFEST_DIR: &'static str = "CARGO_MANIFEST_DIR";

pub fn base_input() -> InputDirectoryPath<'static> {
    InputDirectoryPath(Cow::Owned(
        match image_annealing_cli_util::path::make_base_path_using_environment_variable(
            Path::new(""),
            TEST_UTIL_BASE_INPUT_PATH,
        ) {
            Ok(base_path) => base_path.into_owned(),
            Err(std::env::VarError::NotPresent) => {
                let mut path = PathBuf::new();
                path.push("..");
                path.push("test_data");
                match image_annealing_cli_util::path::make_base_path_using_environment_variable(
                    &path,
                    CARGO_MANIFEST_DIR,
                ) {
                    Ok(base_path) => base_path.into_owned(),
                    Err(std::env::VarError::NotPresent) => {
                        image_annealing_cli_util::path::make_base_path_using_current_directory(
                            &path,
                        )
                        .unwrap()
                        .into_owned()
                    }
                    Err(e) => panic!(
                        "error accessing the {} environment variable, {}",
                        CARGO_MANIFEST_DIR, e
                    ),
                }
            }
            Err(e) => panic!(
                "error accessing the {} environment variable, {}",
                TEST_UTIL_BASE_INPUT_PATH, e
            ),
        },
    ))
}

pub fn base_output() -> OutputDirectoryPath<'static> {
    OutputDirectoryPath(Cow::Owned(
        match image_annealing_cli_util::path::make_base_path_using_environment_variable(
            Path::new(""),
            TEST_UTIL_BASE_OUTPUT_PATH,
        ) {
            Ok(base_path) => base_path.into_owned(),
            Err(std::env::VarError::NotPresent) => {
                let mut path = PathBuf::new();
                path.push("..");
                path.push("test_output");
                match image_annealing_cli_util::path::make_base_path_using_environment_variable(
                    &path,
                    CARGO_MANIFEST_DIR,
                ) {
                    Ok(base_path) => base_path.into_owned(),
                    Err(std::env::VarError::NotPresent) => {
                        image_annealing_cli_util::path::make_base_path_using_current_directory(
                            &path,
                        )
                        .unwrap()
                        .into_owned()
                    }
                    Err(e) => panic!(
                        "error accessing the {} environment variable, {}",
                        CARGO_MANIFEST_DIR, e
                    ),
                }
            }
            Err(e) => panic!(
                "error accessing the {} environment variable, {}",
                TEST_UTIL_BASE_OUTPUT_PATH, e
            ),
        },
    ))
}

pub fn relative_input_file<'a>(relative_path: &'a str) -> UnverifiedInputFilePath<'a> {
    UnverifiedInputFilePath(Cow::Borrowed(RelativePath::new(relative_path)))
}

pub fn absolute_input_file<'a>(relative_path: &'a str) -> InputFilePath<'static> {
    InputFilePath::try_from_with_path_context(relative_input_file(relative_path), base_input().0)
        .unwrap()
}

pub fn relative_input_directory<'a>(relative_path: &'a str) -> UnverifiedInputDirectoryPath<'a> {
    UnverifiedInputDirectoryPath(Cow::Borrowed(RelativePath::new(relative_path)))
}

pub fn absolute_input_directory<'a>(relative_path: &'a str) -> InputDirectoryPath<'static> {
    InputDirectoryPath::try_from_with_path_context(
        relative_input_directory(relative_path),
        base_input().0,
    )
    .unwrap()
}

pub fn relative_output_file<'a>(relative_path: &'a str) -> UnverifiedOutputFilePath<'a> {
    UnverifiedOutputFilePath(Cow::Borrowed(RelativePath::new(relative_path)))
}

pub fn absolute_output_file<'a>(relative_path: &'a str) -> OutputFilePath<'static> {
    OutputFilePath::try_from_with_path_context(relative_output_file(relative_path), base_output().0)
        .unwrap()
}

pub fn relative_output_directory<'a>(relative_path: &'a str) -> UnverifiedOutputDirectoryPath<'a> {
    UnverifiedOutputDirectoryPath(Cow::Borrowed(RelativePath::new(relative_path)))
}

pub fn absolute_output_directory<'a>(relative_path: &'a str) -> OutputDirectoryPath<'static> {
    OutputDirectoryPath::try_from_with_path_context(
        relative_output_directory(relative_path),
        base_output().0,
    )
    .unwrap()
}
