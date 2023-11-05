use relative_path::{RelativePath, RelativePathBuf};
use std::error::Error;
use std::path::{Path, PathBuf};

mod base_path;

pub use base_path::{
    make_base_path, make_base_path_using_current_directory,
    make_base_path_using_environment_variable,
};

pub trait FromWithPathContext<T: ?Sized>: Sized {
    fn from_with_path_context<P: AsRef<Path>>(value: T, base_path: P) -> Self;
}

pub trait IntoWithPathContext<T>: Sized {
    fn into_with_path_context<P: AsRef<Path>>(self, base_path: P) -> T;
}

impl<T, U> IntoWithPathContext<U> for T
where
    U: FromWithPathContext<T>,
{
    fn into_with_path_context<P: AsRef<Path>>(self, base_path: P) -> U {
        <U as FromWithPathContext<T>>::from_with_path_context(self, base_path)
    }
}

pub trait TryFromWithPathContext<T: ?Sized>: Sized {
    type Error;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: T,
        base_path: P,
    ) -> Result<Self, Self::Error>;
}

pub trait TryIntoWithPathContext<T>: Sized {
    type Error;

    fn try_into_with_path_context<P: AsRef<Path>>(self, base_path: P) -> Result<T, Self::Error>;
}

impl<T, U> TryIntoWithPathContext<U> for T
where
    U: TryFromWithPathContext<T>,
{
    type Error = <U as TryFromWithPathContext<T>>::Error;

    fn try_into_with_path_context<P: AsRef<Path>>(self, base_path: P) -> Result<U, Self::Error> {
        <U as TryFromWithPathContext<T>>::try_from_with_path_context(self, base_path)
    }
}

impl FromWithPathContext<&RelativePath> for PathBuf {
    fn from_with_path_context<P: AsRef<Path>>(value: &RelativePath, base_path: P) -> Self {
        value.to_path(base_path)
    }
}

impl<T: AsRef<Path>> TryFromWithPathContext<T> for RelativePathBuf {
    type Error = Box<dyn Error>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: T,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(Self::from_path(value.as_ref().strip_prefix(base_path)?)?)
    }
}

#[cfg(test)]
mod tests;
