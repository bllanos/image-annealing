use crate::TestError;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub enum ReadAction<T> {
    Data(T),
    IoError(TestError),
}

impl<T> ReadAction<T> {
    pub fn from_data(data: T) -> Self {
        Self::Data(data)
    }

    pub fn from_error<E: Into<TestError>>(error: E) -> Self {
        Self::IoError(error.into())
    }
}

impl<T: Deref> ReadAction<T>
where
    <T as Deref>::Target: AsRef<[u8]>,
{
    pub fn into_reader(self) -> impl std::io::Read {
        match self {
            Self::Data(data) => Reader::Data(ReaderData::from(data)),
            Self::IoError(error) => Reader::IoError(Some(error)),
        }
    }
}

impl<T: Clone> Clone for ReadAction<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Data(data) => Self::Data(data.clone()),
            Self::IoError(error) => Self::IoError(error.clone()),
        }
    }
}

struct ReaderData<T> {
    count_bytes_read: usize,
    data: T,
}

impl<T> From<T> for ReaderData<T> {
    fn from(value: T) -> Self {
        Self {
            count_bytes_read: 0,
            data: value,
        }
    }
}

impl<T: Deref> std::io::Read for ReaderData<T>
where
    <T as Deref>::Target: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let slice = self.data.as_ref();
        if slice.len() > self.count_bytes_read {
            let new_bytes_read =
                <&[u8] as std::io::Read>::read(&mut &slice[self.count_bytes_read..], buf)?;
            self.count_bytes_read += new_bytes_read;
            std::io::Result::Ok(new_bytes_read)
        } else {
            std::io::Result::Ok(0)
        }
    }
}

enum Reader<T> {
    Data(ReaderData<T>),
    IoError(Option<TestError>),
}

impl<T: Deref> std::io::Read for Reader<T>
where
    <T as Deref>::Target: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Data(reader_data) => reader_data.read(buf),
            Self::IoError(error) => std::io::Result::Err(std::io::Error::other(
                error
                    .take()
                    .unwrap_or_else(|| "read attempted after an error".into()),
            )),
        }
    }
}

#[derive(Clone)]
pub enum WriteAction {
    Data,
    IoError(TestError),
}

impl WriteAction {
    pub fn from_data() -> Self {
        Self::Data
    }

    pub fn from_error<E: Into<TestError>>(error: E) -> Self {
        Self::IoError(error.into())
    }

    pub fn into_writer(self) -> WriterObserverPair<impl std::io::Write> {
        match self {
            Self::Data => {
                let data = Rc::new(RefCell::new(Vec::new()));
                WriterObserverPair {
                    writer: Writer::Data(Rc::clone(&data)),
                    observer: WriteObserver::Data(data),
                }
            }
            Self::IoError(error) => WriterObserverPair {
                writer: Writer::IoError(Some(error)),
                observer: WriteObserver::IoError,
            },
        }
    }
}

pub struct WriterObserverPair<T: std::io::Write> {
    pub writer: T,
    pub observer: WriteObserver,
}

enum Writer {
    Data(Rc<RefCell<Vec<u8>>>),
    IoError(Option<TestError>),
}

impl std::io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Data(ref data) => data.borrow_mut().write(buf),
            Self::IoError(error) => std::io::Result::Err(std::io::Error::other(
                error
                    .take()
                    .unwrap_or_else(|| "write attempted after an error".into()),
            )),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Data(ref data) => data.borrow_mut().flush(),
            Self::IoError(error) => std::io::Result::Err(std::io::Error::other(
                error
                    .take()
                    .unwrap_or_else(|| "flush attempted after an error".into()),
            )),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum WriteObserver {
    Data(Rc<RefCell<Vec<u8>>>),
    IoError,
}

impl<T: Deref + std::fmt::Debug + Eq> PartialEq<WriteContent<T>> for WriteObserver
where
    <T as Deref>::Target: AsRef<[u8]>,
{
    fn eq(&self, other: &WriteContent<T>) -> bool {
        match self {
            Self::Data(data) => match other {
                WriteContent::Data(other_data) => data.borrow().as_slice() == other_data.as_ref(),
                WriteContent::IoError => false,
            },
            Self::IoError => match other {
                WriteContent::Data(_) => false,
                WriteContent::IoError => true,
            },
        }
    }
}

impl<T: Deref + std::fmt::Debug + Eq> PartialEq<WriteObserver> for WriteContent<T>
where
    <T as Deref>::Target: AsRef<[u8]>,
{
    fn eq(&self, other: &WriteObserver) -> bool {
        other == self
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum WriteContent<T: std::fmt::Debug + Eq> {
    Data(T),
    IoError,
}
