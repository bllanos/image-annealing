use std::path::{Path, PathBuf};

pub fn make_test_data_path<T, P>(components: T) -> PathBuf
where
    T: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut path = PathBuf::new();
    path.push("..");
    path.push("test_data");
    for c in components.into_iter() {
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
    for c in components.into_iter() {
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
