use image_annealing_cli_util::path::{
    FromWithPathContext, InputDirectoryPath, InputFilePath, OutputDirectoryPath, OutputFilePath,
    TryFromWithPathContext, UnverifiedInputDirectoryPath, UnverifiedInputFilePath,
    UnverifiedOutputDirectoryPath, UnverifiedOutputFilePath,
};
use relative_path::RelativePath;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

const TEST_UTIL_ROOT_PATH: &str = "TEST_UTIL_ROOT_PATH";

const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

fn make_base_path(suffix: &str) -> Cow<'static, Path> {
    Cow::Owned(
        match image_annealing_cli_util::path::make_base_path_using_environment_variable(
            Path::new(""),
            TEST_UTIL_ROOT_PATH,
        ) {
            Ok(base_path) => {
                let mut path = base_path.into_owned();
                path.push(suffix);
                path
            }
            Err(std::env::VarError::NotPresent) => {
                let mut path = PathBuf::new();
                path.push("..");
                path.push(suffix);
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
                TEST_UTIL_ROOT_PATH, e
            ),
        },
    )
}

pub fn base_input() -> InputDirectoryPath<'static> {
    InputDirectoryPath(make_base_path("test_data"))
}

pub fn base_output() -> OutputDirectoryPath<'static> {
    OutputDirectoryPath(make_base_path("test_output"))
}

pub fn relative_input_file(relative_path: &str) -> UnverifiedInputFilePath {
    UnverifiedInputFilePath(Cow::Borrowed(RelativePath::new(relative_path)))
}

pub fn absolute_input_file(relative_path: &str) -> InputFilePath<'static> {
    InputFilePath::try_from_with_path_context(relative_input_file(relative_path), base_input().0)
        .unwrap()
}

pub fn relative_input_directory(relative_path: &str) -> UnverifiedInputDirectoryPath {
    UnverifiedInputDirectoryPath(Cow::Borrowed(RelativePath::new(relative_path)))
}

pub fn absolute_input_directory(relative_path: &str) -> InputDirectoryPath<'static> {
    InputDirectoryPath::try_from_with_path_context(
        relative_input_directory(relative_path),
        base_input().0,
    )
    .unwrap()
}

pub fn unverified_absolute_input_path(relative_path: &str) -> PathBuf {
    PathBuf::from_with_path_context(RelativePath::new(relative_path), base_input().0)
}

pub fn relative_output_file(relative_path: &str) -> UnverifiedOutputFilePath<'static> {
    UnverifiedOutputFilePath(Cow::Owned(
        RelativePath::new(relative_path).to_relative_path_buf(),
    ))
}

pub fn absolute_output_file(relative_path: &str) -> OutputFilePath<'static> {
    OutputFilePath::try_from_with_path_context(relative_output_file(relative_path), base_output().0)
        .unwrap()
}

pub fn relative_output_directory(relative_path: &str) -> UnverifiedOutputDirectoryPath<'static> {
    UnverifiedOutputDirectoryPath(Cow::Owned(
        RelativePath::new(relative_path).to_relative_path_buf(),
    ))
}

pub fn absolute_output_directory(relative_path: &str) -> OutputDirectoryPath<'static> {
    OutputDirectoryPath::try_from_with_path_context(
        relative_output_directory(relative_path),
        base_output().0,
    )
    .unwrap()
}

pub fn unverified_absolute_output_path(relative_path: &str) -> PathBuf {
    PathBuf::from_with_path_context(RelativePath::new(relative_path), base_output().0)
}

pub fn filepath_to_filename(path: &str) -> String {
    Path::new(path)
        .with_extension("")
        .to_str()
        .unwrap()
        .replace(std::path::MAIN_SEPARATOR, "-")
}

#[macro_export]
macro_rules! unique_relative_output_file {
    () => {{
        test_util::path::relative_output_file(&format!(
            "{}-line{}-column{}",
            test_util::path::filepath_to_filename(file!()),
            line!(),
            column!()
        ))
    }};
}

#[macro_export]
macro_rules! unique_absolute_output_file {
    () => {{
        test_util::path::absolute_output_file(&format!(
            "{}-line{}-column{}",
            test_util::path::filepath_to_filename(file!()),
            line!(),
            column!()
        ))
    }};
}

#[macro_export]
macro_rules! unique_absolute_output_directory {
    () => {{
        test_util::path::absolute_output_directory(&format!(
            "{}-line{}-column{}",
            test_util::path::filepath_to_filename(file!()),
            line!(),
            column!()
        ))
    }};
}
