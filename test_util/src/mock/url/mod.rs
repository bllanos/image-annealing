use crate::TestError;
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub enum UrlContent<'a> {
    String(Cow<'a, str>),
    Bytes(Cow<'a, [u8]>),
    IoError,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UrlWithContent<'a, T: std::fmt::Debug + Eq + ToOwned + ?Sized>
where
    <T as ToOwned>::Owned: std::fmt::Debug,
{
    pub url: Cow<'a, T>,
    pub content: UrlContent<'a>,
}

struct ReadableUrlReader<'a, T: std::fmt::Debug + Eq + ToOwned + ?Sized>
where
    <T as ToOwned>::Owned: std::fmt::Debug,
{
    count_bytes_read: usize,
    data: &'a UrlWithContent<'a, T>,
}

impl<'a, T: std::fmt::Debug + Eq + ToOwned + ?Sized> ReadableUrlReader<'a, T>
where
    <T as ToOwned>::Owned: std::fmt::Debug,
{
    fn new(data: &'a UrlWithContent<'a, T>) -> Self {
        Self {
            count_bytes_read: 0,
            data,
        }
    }
}

impl<'a, T: std::fmt::Debug + Eq + ToOwned + ?Sized> std::io::Read for ReadableUrlReader<'a, T>
where
    <T as ToOwned>::Owned: std::fmt::Debug,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let slice = match self.data.content {
            UrlContent::String(ref string) => std::io::Result::Ok(string.as_bytes()),
            UrlContent::Bytes(ref data) => std::io::Result::Ok(data.as_ref()),
            UrlContent::IoError => std::io::Result::Err(std::io::Error::other(TestError(format!(
                "error reading from mock url {:?}",
                self.data.url.as_ref()
            )))),
        }?;

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

pub struct ReadableUrlOpener<'a, T: std::fmt::Debug + Eq + ToOwned + ?Sized>
where
    <T as ToOwned>::Owned: std::fmt::Debug,
{
    urls: &'a [UrlWithContent<'a, T>],
    url_read_records: Vec<<T as ToOwned>::Owned>,
}

impl<'a, T: std::fmt::Debug + Eq + ToOwned + ?Sized> ReadableUrlOpener<'a, T>
where
    <T as ToOwned>::Owned: std::fmt::Debug,
{
    pub fn new(urls: &'a [UrlWithContent<'a, T>]) -> Self {
        Self {
            urls,
            url_read_records: Vec::new(),
        }
    }

    pub fn open_url(&mut self, url: &T) -> std::io::Result<impl 'a + std::io::Read> {
        self.url_read_records.push(url.to_owned());
        match self
            .urls
            .iter()
            .find(|available_url| url == available_url.url.as_ref())
        {
            Some(available_url) => std::io::Result::Ok(ReadableUrlReader::new(available_url)),
            None => std::io::Result::Err(std::io::Error::other(TestError(format!(
                "URL {url:?} is not in the set of mock readable URLs",
            )))),
        }
    }

    pub fn url_read_records(&self) -> &[<T as ToOwned>::Owned] {
        &self.url_read_records
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WritableUrlAction {
    Data,
    IoError,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WritableUrlActionResult {
    Data(Rc<RefCell<Vec<u8>>>),
    IoError,
}

impl From<WritableUrlAction> for WritableUrlActionResult {
    fn from(value: WritableUrlAction) -> Self {
        match value {
            WritableUrlAction::Data => Self::Data(Rc::new(RefCell::new(Vec::new()))),
            WritableUrlAction::IoError => Self::IoError,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct WritableUrl<'a, T: std::fmt::Debug + Eq + ToOwned + ?Sized>
where
    <T as ToOwned>::Owned: std::fmt::Debug,
{
    pub url: Cow<'a, T>,
    pub action: WritableUrlAction,
}

#[derive(Debug, Eq, PartialEq)]
struct WritableUrlResult<T: std::fmt::Debug + Eq> {
    url: T,
    action: WritableUrlActionResult,
}

struct WritableUrlWriter<'a, T: std::fmt::Debug + ?Sized> {
    url: &'a T,
    action: WritableUrlActionResult,
}

impl<'a, T: std::fmt::Debug + ?Sized> std::io::Write for WritableUrlWriter<'a, T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.action {
            WritableUrlActionResult::Data(ref data) => data.borrow_mut().write(buf),
            WritableUrlActionResult::IoError => std::io::Result::Err(std::io::Error::other(
                TestError(format!("error writing to mock url {:?}", self.url)),
            )),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self.action {
            WritableUrlActionResult::Data(ref data) => data.borrow_mut().flush(),
            WritableUrlActionResult::IoError => std::io::Result::Err(std::io::Error::other(
                TestError(format!("error flushing to mock url {:?}", self.url)),
            )),
        }
    }
}

#[derive(Debug, Eq)]
pub enum UrlWriteRecord<T: std::fmt::Debug + Eq, U: std::fmt::Debug + Eq> {
    WriteResult(T),
    Inaccessible(U),
}

pub struct WritableUrlOpener<'a, T: std::fmt::Debug + Eq + ToOwned + ?Sized>
where
    <T as ToOwned>::Owned: std::fmt::Debug + Eq,
{
    urls: &'a [WritableUrl<'a, T>],
    url_write_records:
        Vec<UrlWriteRecord<WritableUrlResult<<T as ToOwned>::Owned>, <T as ToOwned>::Owned>>,
}

impl<'a, T: std::fmt::Debug + Eq + ToOwned + ?Sized> WritableUrlOpener<'a, T>
where
    <T as ToOwned>::Owned: std::fmt::Debug + Eq,
{
    pub fn new(urls: &'a [WritableUrl<'a, T>]) -> Self {
        Self {
            urls,
            url_write_records: Vec::new(),
        }
    }

    pub fn open_url(&mut self, url: &T) -> std::io::Result<impl 'a + std::io::Write> {
        let owned_url = url.to_owned();
        match self
            .urls
            .iter()
            .find(|ref available_url| url == available_url.url.as_ref())
        {
            Some(available_url) => {
                let writable_url_result = WritableUrlResult {
                    url: owned_url,
                    action: available_url.action.into(),
                };
                let writable_url_writer = WritableUrlWriter {
                    url: &available_url.url,
                    action: writable_url_result.action.clone(),
                };
                self.url_write_records
                    .push(UrlWriteRecord::WriteResult(writable_url_result));
                std::io::Result::Ok(writable_url_writer)
            }

            None => {
                self.url_write_records
                    .push(UrlWriteRecord::Inaccessible(owned_url));
                std::io::Result::Err(std::io::Error::other(TestError(format!(
                    "URL {url:?} is not in the set of mock writable URLs"
                ))))
            }
        }
    }

    pub fn url_write_records<U, V>(
        &self,
    ) -> &[impl PartialEq<UrlWriteRecord<UrlWithContent<'_, U>, V>> + std::fmt::Debug + Eq]
    where
        U: std::fmt::Debug + Eq + ToOwned + ?Sized,
        <U as ToOwned>::Owned: std::fmt::Debug,
        V: std::fmt::Debug + Eq,
        <T as ToOwned>::Owned: PartialEq<V> + PartialEq<U>,
    {
        &self.url_write_records
    }

    pub fn is_url_write_records_empty(&self) -> bool {
        self.url_write_records.is_empty()
    }
}

impl<'a> PartialEq<UrlContent<'a>> for WritableUrlActionResult {
    fn eq(&self, other: &UrlContent<'a>) -> bool {
        match self {
            Self::Data(data) => match other {
                UrlContent::String(other_string) => {
                    data.borrow().as_slice() == other_string.as_bytes()
                }
                UrlContent::Bytes(other_data) => data.borrow().as_slice() == other_data.as_ref(),
                UrlContent::IoError => false,
            },
            Self::IoError => match other {
                UrlContent::String(_) | UrlContent::Bytes(_) => false,
                UrlContent::IoError => true,
            },
        }
    }
}

impl<'a, T: std::fmt::Debug + Eq + PartialEq<U>, U: std::fmt::Debug + Eq + ToOwned + ?Sized>
    PartialEq<UrlWithContent<'a, U>> for WritableUrlResult<T>
where
    <U as ToOwned>::Owned: std::fmt::Debug,
{
    fn eq(&self, other: &UrlWithContent<'a, U>) -> bool {
        &self.url == other.url.as_ref() && self.action == other.content
    }
}

impl<
        T: std::fmt::Debug + Eq,
        U: std::fmt::Debug + Eq,
        V: std::fmt::Debug + Eq,
        W: std::fmt::Debug + Eq,
    > PartialEq<UrlWriteRecord<V, W>> for UrlWriteRecord<T, U>
where
    T: PartialEq<V>,
    U: PartialEq<W>,
{
    fn eq(&self, other: &UrlWriteRecord<V, W>) -> bool {
        match self {
            Self::WriteResult(write_result) => match other {
                UrlWriteRecord::WriteResult(other_write_result) => {
                    write_result == other_write_result
                }
                UrlWriteRecord::Inaccessible(_) => false,
            },
            Self::Inaccessible(inner) => match other {
                UrlWriteRecord::WriteResult(_) => false,
                UrlWriteRecord::Inaccessible(other_inner) => inner == other_inner,
            },
        }
    }
}
