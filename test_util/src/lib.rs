use std::path::{Path, PathBuf};

pub mod algorithm;
pub mod displacement_goal;
pub mod image;
pub mod operation;
pub mod permutation;

pub fn make_test_data_path<T, P>(components: T) -> PathBuf
where
    T: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut path = PathBuf::new();
    path.push("..");
    path.push("test_data");
    for c in components {
        path.push(c);
    }
    path
}

pub fn make_test_data_path_string<T, P>(components: T) -> String
where
    T: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let path = make_test_data_path(components);
    String::from(path.to_str().unwrap())
}

pub fn make_test_output_path<T, P>(components: T) -> PathBuf
where
    T: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut path = PathBuf::new();
    path.push("..");
    path.push("test_output");
    for c in components {
        path.push(c);
    }
    path
}

pub fn make_test_output_path_string<T, P>(components: T) -> String
where
    T: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let path = make_test_output_path(components);
    String::from(path.to_str().unwrap())
}

pub fn assert_error_contains<T: std::fmt::Debug, E: std::fmt::Display>(
    result: Result<T, E>,
    message: &str,
) {
    let e = result.unwrap_err();
    if !e.to_string().contains(message) {
        eprintln!("error message: {}", e);
        eprintln!("expected substring: {}", message);
        panic!("error message does not contain the expected substring")
    }
}
