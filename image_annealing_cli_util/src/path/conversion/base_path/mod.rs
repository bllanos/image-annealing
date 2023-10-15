use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Path;

pub fn make_base_path<P: AsRef<Path>, E, O>(
    candidate_path: &Path,
    make_context_path: O,
) -> Result<Cow<Path>, E>
where
    O: FnOnce() -> Result<P, E>,
{
    Ok(if candidate_path.is_absolute() {
        Cow::Borrowed(candidate_path)
    } else {
        Cow::Owned(make_context_path()?.as_ref().join(candidate_path))
    })
}

pub fn make_base_path_using_current_directory(
    candidate_path: &Path,
) -> Result<Cow<Path>, std::io::Error> {
    make_base_path(candidate_path, std::env::current_dir)
}

pub fn make_base_path_using_environment_variable<T: AsRef<OsStr>>(
    candidate_path: &Path,
    environment_variable_key: T,
) -> Result<Cow<Path>, std::env::VarError> {
    make_base_path(candidate_path, || std::env::var(environment_variable_key))
}

#[cfg(test)]
mod tests;
