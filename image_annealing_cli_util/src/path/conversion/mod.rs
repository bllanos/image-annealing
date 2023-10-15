use relative_path::{RelativePath, RelativePathBuf};
use std::error::Error;
use std::path::{Path, PathBuf};

mod base_path;

pub use base_path::{
    make_base_path, make_base_path_using_current_directory,
    make_base_path_using_environment_variable,
};

pub trait FromWithPathContext<T: ?Sized, P: AsRef<Path>>: Sized {
    fn from_with_path_context(value: T, base_path: P) -> Self;
}

pub trait IntoWithPathContext<T, P: AsRef<Path>>: Sized {
    fn into_with_path_context(self, base_path: P) -> T;
}

impl<T, U, P> IntoWithPathContext<U, P> for T
where
    P: AsRef<Path>,
    U: FromWithPathContext<T, P>,
{
    fn into_with_path_context(self, base_path: P) -> U {
        <U as FromWithPathContext<T, P>>::from_with_path_context(self, base_path)
    }
}

pub trait TryFromWithPathContext<T: ?Sized, P: AsRef<Path>>: Sized {
    type Error;

    fn try_from_with_path_context(value: T, base_path: P) -> Result<Self, Self::Error>;
}

pub trait TryIntoWithPathContext<T, P: AsRef<Path>>: Sized {
    type Error;

    fn try_into_with_path_context(self, base_path: P) -> Result<T, Self::Error>;
}

impl<T, U, P> TryIntoWithPathContext<U, P> for T
where
    P: AsRef<Path>,
    U: TryFromWithPathContext<T, P>,
{
    type Error = <U as TryFromWithPathContext<T, P>>::Error;

    fn try_into_with_path_context(self, base_path: P) -> Result<U, Self::Error> {
        <U as TryFromWithPathContext<T, P>>::try_from_with_path_context(self, base_path)
    }
}

impl<P: AsRef<Path>> FromWithPathContext<&RelativePath, P> for PathBuf {
    fn from_with_path_context(value: &RelativePath, base_path: P) -> Self {
        value.to_path(base_path)
    }
}

impl<T: AsRef<Path>, P: AsRef<Path>> TryFromWithPathContext<T, P> for RelativePathBuf {
    type Error = Box<dyn Error>;

    fn try_from_with_path_context(value: T, base_path: P) -> Result<Self, Self::Error> {
        Ok(Self::from_path(value.as_ref().strip_prefix(base_path)?)?)
    }
}

#[cfg(test)]
mod tests;
