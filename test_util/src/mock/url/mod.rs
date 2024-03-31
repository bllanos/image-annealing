use super::io::{ReadAction, WriteAction, WriteObserver, WriterObserverPair};
use crate::TestError;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Deref;

pub struct UrlReadAction<Url, Data> {
    pub url: Url,
    pub action: ReadAction<Data>,
}

pub struct ReadableUrlOpener<Url: Deref, Data>
where
    <Url as Deref>::Target: ToOwned,
{
    urls: HashMap<Url, ReadAction<Data>>,
    url_reads: HashSet<<<Url as Deref>::Target as ToOwned>::Owned>,
}

impl<Url: Deref + Hash + Eq, Data> ReadableUrlOpener<Url, Data>
where
    <Url as Deref>::Target: ToOwned,
{
    pub fn new<I: IntoIterator<Item = UrlReadAction<Url, Data>>>(urls: I) -> Self {
        Self {
            urls: HashMap::from_iter(
                urls.into_iter()
                    .map(|url_read_action| (url_read_action.url, url_read_action.action)),
            ),
            url_reads: HashSet::new(),
        }
    }
}

impl<Url: Deref, Data> ReadableUrlOpener<Url, Data>
where
    <Url as Deref>::Target: ToOwned,
{
    pub fn is_url_reads_empty(&self) -> bool {
        self.url_reads.is_empty()
    }
}

impl<Url: Deref + Hash + Eq, Data: Deref + Clone> ReadableUrlOpener<Url, Data>
where
    Url: Borrow<<Url as Deref>::Target>,
    <Url as Deref>::Target: ToOwned + std::fmt::Debug + Hash + Eq,
    <<Url as Deref>::Target as ToOwned>::Owned: Borrow<<Url as Deref>::Target> + Hash + Eq,
    <Data as Deref>::Target: AsRef<[u8]>,
{
    pub fn open_url(
        &mut self,
        url: &<Url as Deref>::Target,
    ) -> std::io::Result<impl std::io::Read> {
        self.url_reads.insert(url.to_owned());
        match self.urls.get(url) {
            Some(content) => std::io::Result::Ok(content.clone().into_reader()),
            None => std::io::Result::Err(std::io::Error::other(TestError(format!(
                "URL {url:?} is inaccessible for reading",
            )))),
        }
    }
}

impl<Url: Deref, Data> ReadableUrlOpener<Url, Data>
where
    <Url as Deref>::Target: ToOwned + Hash + Eq,
    <<Url as Deref>::Target as ToOwned>::Owned: Borrow<<Url as Deref>::Target> + Hash + Eq,
{
    pub fn is_url_reads_set_equal<I: IntoIterator>(&self, contents: I) -> bool
    where
        <I as IntoIterator>::Item: AsRef<<Url as Deref>::Target>,
    {
        contents
            .into_iter()
            .all(|item| self.url_reads.contains(item.as_ref()))
    }
}

pub struct UrlWriteAction<Url> {
    pub url: Url,
    pub action: WriteAction,
}

#[derive(Debug, Eq)]
pub enum UrlWriteContent<T: std::fmt::Debug + Eq> {
    Content(T),
    Inaccessible,
}

#[derive(Debug, Eq)]
pub struct UrlWrite<Url: Deref + std::fmt::Debug + Eq, T: std::fmt::Debug + Eq>
where
    <Url as Deref>::Target: Eq,
{
    pub url: Url,
    pub outcome: UrlWriteContent<T>,
}

pub struct WritableUrlOpener<Url: Deref>
where
    <Url as Deref>::Target: ToOwned,
    <<Url as Deref>::Target as ToOwned>::Owned: Deref + std::fmt::Debug + Eq,
    <<<Url as Deref>::Target as ToOwned>::Owned as Deref>::Target: Eq,
{
    urls: HashMap<Url, WriteAction>,
    url_writes: Vec<UrlWrite<<<Url as Deref>::Target as ToOwned>::Owned, WriteObserver>>,
}

impl<Url: Deref + Hash + Eq> WritableUrlOpener<Url>
where
    <Url as Deref>::Target: ToOwned,
    <<Url as Deref>::Target as ToOwned>::Owned: Deref + std::fmt::Debug + Eq,
    <<<Url as Deref>::Target as ToOwned>::Owned as Deref>::Target: Eq,
{
    pub fn new<I: IntoIterator<Item = UrlWriteAction<Url>>>(urls: I) -> Self {
        Self {
            urls: HashMap::from_iter(
                urls.into_iter()
                    .map(|url_write_action| (url_write_action.url, url_write_action.action)),
            ),
            url_writes: Vec::new(),
        }
    }
}

impl<Url: Deref> WritableUrlOpener<Url>
where
    <Url as Deref>::Target: ToOwned,
    <<Url as Deref>::Target as ToOwned>::Owned: Deref + std::fmt::Debug + Eq,
    <<<Url as Deref>::Target as ToOwned>::Owned as Deref>::Target: Eq,
{
    pub fn is_url_writes_empty(&self) -> bool {
        self.url_writes.is_empty()
    }
}

impl<Url: Deref + Hash + Eq> WritableUrlOpener<Url>
where
    Url: Borrow<<Url as Deref>::Target>,
    <Url as Deref>::Target: ToOwned + Hash + Eq + std::fmt::Debug,
    <<Url as Deref>::Target as ToOwned>::Owned: Deref + std::fmt::Debug + Eq,
    <<<Url as Deref>::Target as ToOwned>::Owned as Deref>::Target: Eq,
{
    pub fn open_url(
        &mut self,
        url: &<Url as Deref>::Target,
    ) -> std::io::Result<impl std::io::Write> {
        let owned_url = url.to_owned();

        match self.urls.get(url) {
            Some(content) => {
                let WriterObserverPair { writer, observer } = content.clone().into_writer();
                self.url_writes.push(UrlWrite {
                    url: owned_url,
                    outcome: UrlWriteContent::Content(observer),
                });
                std::io::Result::Ok(writer)
            }
            None => {
                self.url_writes.push(UrlWrite {
                    url: owned_url,
                    outcome: UrlWriteContent::Inaccessible,
                });
                std::io::Result::Err(std::io::Error::other(TestError(format!(
                    "URL {url:?} is inaccessible for writing",
                ))))
            }
        }
    }
}

impl<Url: Deref> WritableUrlOpener<Url>
where
    <Url as Deref>::Target: ToOwned,
    <<Url as Deref>::Target as ToOwned>::Owned: Deref + std::fmt::Debug + Eq,
    <<<Url as Deref>::Target as ToOwned>::Owned as Deref>::Target: Eq,
{
    pub fn is_url_writes_equal<'a, I: IntoIterator>(&'a self, contents: I) -> bool
    where
        <I as IntoIterator>::Item:
            PartialEq<&'a UrlWrite<<<Url as Deref>::Target as ToOwned>::Owned, WriteObserver>>,
    {
        contents.into_iter().eq(self.url_writes.iter())
    }
}

impl<T: std::fmt::Debug + Eq, U: std::fmt::Debug + Eq> PartialEq<UrlWriteContent<U>>
    for UrlWriteContent<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &UrlWriteContent<U>) -> bool {
        match self {
            Self::Content(content) => match other {
                UrlWriteContent::Content(other_content) => content == other_content,
                UrlWriteContent::Inaccessible => false,
            },
            Self::Inaccessible => match other {
                UrlWriteContent::Content(_) => false,
                UrlWriteContent::Inaccessible => true,
            },
        }
    }
}

impl<
        Url1: Deref + std::fmt::Debug + Eq,
        Url2: Deref + std::fmt::Debug + Eq,
        T1: std::fmt::Debug + Eq,
        T2: std::fmt::Debug + Eq,
    > PartialEq<UrlWrite<Url2, T2>> for UrlWrite<Url1, T1>
where
    <Url1 as Deref>::Target: Eq + PartialEq<<Url2 as Deref>::Target>,
    <Url2 as Deref>::Target: Eq,
    T1: PartialEq<T2>,
{
    fn eq(&self, other: &UrlWrite<Url2, T2>) -> bool {
        *self.url == *other.url && self.outcome == other.outcome
    }
}
