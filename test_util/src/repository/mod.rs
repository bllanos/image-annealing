use crate::{TestError, TestResult};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

pub fn make_documentation_path() -> std::path::PathBuf {
    let mut path = PathBuf::new();
    path.push("..");
    path.push("docs");
    path
}

pub fn check_that_file_is_current_and_create_new_file<
    P: AsRef<Path>,
    Reader: Read,
    Writer: Write,
    OpenFile: FnMut(&Path) -> std::io::Result<Reader>,
    CreateFile: FnMut(&Path) -> std::io::Result<Writer>,
>(
    mut open_file: OpenFile,
    mut create_file: CreateFile,
    file_path: P,
    file_extension: &str,
    file_content: &str,
) -> TestResult {
    let path = file_path.as_ref().with_extension(file_extension);

    let existing_file = open_file(&path).map_err(|why| {
        TestError(format!(
            "failed to open file to check for up-to-date content at path {}: {}",
            path.display(),
            why
        ))
    })?;

    let existing_lines = BufReader::new(existing_file).lines();
    let mut unmatched_line = None;
    for (index, (existing_line_result, updated_line)) in
        existing_lines.zip(file_content.lines()).enumerate()
    {
        match existing_line_result {
            Ok(existing_line) => {
                if existing_line != updated_line {
                    unmatched_line = Some(index);
                    break;
                }
            }
            Err(why) => {
                return TestResult::Err(TestError(format!(
                    "failed to read line {} from file to check for up-to-date content at path {}: {}",
                    index,
                    path.display(),
                    why
                )));
            }
        }
    }

    if let Some(index) = unmatched_line {
        let updated_file_path = path
            .as_path()
            .with_extension(format!("{}.new", file_extension));

        let mut updated_file = create_file(&updated_file_path).map_err(|why| {
            TestError(format!(
                "failed to open an updated file at path {}: {}",
                updated_file_path.display(),
                why
            ))
        })?;

        updated_file
            .write_all(file_content.as_bytes())
            .map_err(|why| {
                TestError(format!(
                    "failed to write to an updated file at path {}: {}",
                    updated_file_path.display(),
                    why
                ))
            })?;

        TestResult::Err(TestError(format!(
            "{} is out of date and an updated version was saved, {}, to be used to overwrite it. The first line that did not match has index {}",
            path.display(),
            updated_file_path.display(),
            index
        )))
    } else {
        TestResult::Ok(())
    }
}

#[cfg(test)]
mod tests;
