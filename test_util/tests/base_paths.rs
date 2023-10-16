use image_annealing_cli_util::path::{InputDirectoryPath, OutputDirectoryPath};
use std::borrow::Cow;
use std::env;
use std::path::{Path, PathBuf};
use test_util::path::{base_input, base_output};

const TEST_UTIL_ROOT_PATH: &str = "TEST_UTIL_ROOT_PATH";

const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

fn path_from_components<T, P>(components: T) -> PathBuf
where
    T: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut path = PathBuf::new();
    for c in components {
        path.push(c);
    }
    path
}

#[test]
fn base_paths() {
    // Environment variable manipulation needs to be performed in an integration test,
    // which is not run in parallel with other tests.
    // See <https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests>
    env::set_var(TEST_UTIL_ROOT_PATH, &path_from_components(["one", "two"]));
    env::set_var(CARGO_MANIFEST_DIR, &path_from_components(["three", "four"]));

    assert_eq!(
        base_input(),
        InputDirectoryPath(Cow::Owned(path_from_components([
            "one",
            "two",
            "test_data"
        ])))
    );
    assert_eq!(
        base_output(),
        OutputDirectoryPath(Cow::Owned(path_from_components([
            "one",
            "two",
            "test_output"
        ])))
    );

    env::remove_var(TEST_UTIL_ROOT_PATH);
    assert_eq!(
        base_input(),
        InputDirectoryPath(Cow::Owned(path_from_components([
            "three",
            "four",
            "..",
            "test_data"
        ])))
    );
    assert_eq!(
        base_output(),
        OutputDirectoryPath(Cow::Owned(path_from_components([
            "three",
            "four",
            "..",
            "test_output"
        ])))
    );

    env::remove_var(CARGO_MANIFEST_DIR);
    let current_directory = env::current_dir().unwrap();
    assert_eq!(
        base_input(),
        InputDirectoryPath(Cow::Owned(
            current_directory.join(path_from_components(["..", "test_data"]))
        ))
    );
    assert_eq!(
        base_output(),
        OutputDirectoryPath(Cow::Owned(
            current_directory.join(path_from_components(["..", "test_output"]))
        ))
    );
}
