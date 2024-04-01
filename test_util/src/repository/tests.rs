mod check_that_file_is_current_and_create_new_file {
    use crate::mock::io::WriteAction;
    use std::path::Path;

    type ReadAction<'a> = crate::mock::io::ReadAction<&'a str>;
    type WriteContent<'a> = crate::mock::io::WriteContent<&'a str>;
    type UrlReadAction<'a> = crate::mock::url::UrlReadAction<&'a Path, &'a str>;
    type UrlWriteAction<'a> = crate::mock::url::UrlWriteAction<&'a Path>;
    type UrlWrite<'a> = crate::mock::url::UrlWrite<&'a Path, WriteContent<'a>>;
    type UrlWriteContent<'a> = crate::mock::url::UrlWriteContent<WriteContent<'a>>;
    type ReadableUrlOpener<'a> = crate::mock::url::ReadableUrlOpener<&'a Path, &'a str>;
    type WritableUrlOpener<'a> = crate::mock::url::WritableUrlOpener<&'a Path>;

    #[test]
    fn file_is_missing_is_an_error() {
        let mut readable_url_opener = ReadableUrlOpener::new([]);
        let mut writable_url_opener = WritableUrlOpener::new([]);
        let file_path = "file path";
        let file_extension = "test";
        let file_content = "file content";

        let result = super::super::check_that_file_is_current_and_create_new_file(
            |path: &Path| readable_url_opener.open_url(path),
            |path: &Path| writable_url_opener.open_url(path),
            file_path,
            file_extension,
            file_content,
        );

        let full_path = Path::new(file_path).with_extension(file_extension);
        assert!(crate::test_result_equals_string(result, &format!("failed to open file to check for up-to-date content at path {}: URL {:?} is inaccessible for reading", full_path.display(), full_path)));
        assert!(readable_url_opener.is_url_reads_set_equal(&[full_path]));
        assert!(writable_url_opener.is_url_writes_empty());
    }

    #[test]
    fn failure_to_read_from_file_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_path = Path::new(file_path).with_extension(file_extension);
        let error = "expected read error";
        let readable_files = [UrlReadAction {
            url: full_path.as_path(),
            action: ReadAction::from_error(error),
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(readable_files);
        let mut writable_url_opener = WritableUrlOpener::new([]);
        let file_content = "file content";

        let result = super::super::check_that_file_is_current_and_create_new_file(
            |path: &Path| readable_url_opener.open_url(path),
            |path: &Path| writable_url_opener.open_url(path),
            file_path,
            file_extension,
            file_content,
        );

        assert!(crate::test_result_equals_string(
            result,
            &format!(
                "failed to read line 0 from file to check for up-to-date content at path {}: {}",
                full_path.display(),
                error
            )
        ));
        assert!(readable_url_opener.is_url_reads_set_equal(&[full_path.as_path()]));
        assert!(writable_url_opener.is_url_writes_empty());
    }

    #[test]
    fn failure_to_open_updated_file_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_input_path = Path::new(file_path).with_extension(file_extension);
        let full_output_path =
            Path::new(file_path).with_extension(format!("{}.new", file_extension));
        let readable_files = [UrlReadAction {
            url: full_input_path.as_path(),
            action: ReadAction::from_data("outdated file content"),
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(readable_files);
        let mut writable_url_opener = WritableUrlOpener::new([]);
        let file_content = "file content";

        let result = super::super::check_that_file_is_current_and_create_new_file(
            |path: &Path| readable_url_opener.open_url(path),
            |path: &Path| writable_url_opener.open_url(path),
            file_path,
            file_extension,
            file_content,
        );

        assert!(crate::test_result_equals_string(
            result,
            &format!(
                "failed to open an updated file at path {}: URL {:?} is inaccessible for writing",
                full_output_path.display(),
                full_output_path
            )
        ));
        assert!(readable_url_opener.is_url_reads_set_equal(&[full_input_path.as_path()]));
        assert!(writable_url_opener.is_url_writes_equal(&[UrlWrite {
            url: full_output_path.as_path(),
            outcome: UrlWriteContent::inaccessible()
        }]));
    }

    #[test]
    fn failure_to_write_to_file_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_input_path = Path::new(file_path).with_extension(file_extension);
        let full_output_path =
            Path::new(file_path).with_extension(format!("{}.new", file_extension));
        let readable_files = [UrlReadAction {
            url: full_input_path.as_path(),
            action: ReadAction::from_data("outdated file content"),
        }];
        let error = "expected write error";
        let writable_files = [UrlWriteAction {
            url: full_output_path.as_path(),
            action: WriteAction::from_error(error),
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(readable_files);
        let mut writable_url_opener = WritableUrlOpener::new(writable_files);
        let file_content = "file content";

        let result = super::super::check_that_file_is_current_and_create_new_file(
            |path: &Path| readable_url_opener.open_url(path),
            |path: &Path| writable_url_opener.open_url(path),
            file_path,
            file_extension,
            file_content,
        );

        assert!(crate::test_result_equals_string(
            result,
            &format!(
                "failed to write to an updated file at path {}: {}",
                full_output_path.display(),
                error
            )
        ));
        assert!(readable_url_opener.is_url_reads_set_equal(&[full_input_path.as_path()]));
        assert!(writable_url_opener.is_url_writes_equal(&[UrlWrite {
            url: full_output_path.as_path(),
            outcome: UrlWriteContent::io_error(),
        }]));
    }

    #[test]
    fn outdated_file_contents_on_first_line_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_input_path = Path::new(file_path).with_extension(file_extension);
        let full_output_path =
            Path::new(file_path).with_extension(format!("{}.new", file_extension));
        let readable_files = [UrlReadAction {
            url: full_input_path.as_path(),
            action: ReadAction::from_data("outdated file content\nmatching line"),
        }];
        let writable_files = [UrlWriteAction {
            url: full_output_path.as_path(),
            action: WriteAction::Data,
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(readable_files);
        let mut writable_url_opener = WritableUrlOpener::new(writable_files);
        let file_content = "file content\nmatching line";

        let result = super::super::check_that_file_is_current_and_create_new_file(
            |path: &Path| readable_url_opener.open_url(path),
            |path: &Path| writable_url_opener.open_url(path),
            file_path,
            file_extension,
            file_content,
        );

        assert!(crate::test_result_equals_string(
            result,
            &format!(
                "{} is out of date and an updated version was saved, {}, to be used to overwrite it. The first line that did not match has index 0",
                full_input_path.display(),
                full_output_path.display()
            )
        ));
        assert!(readable_url_opener.is_url_reads_set_equal(&[full_input_path.as_path()]));
        assert!(writable_url_opener.is_url_writes_equal(&[UrlWrite {
            url: full_output_path.as_path(),
            outcome: UrlWriteContent::from_data(file_content),
        }]));
    }

    #[test]
    fn outdated_file_contents_on_second_line_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_input_path = Path::new(file_path).with_extension(file_extension);
        let full_output_path =
            Path::new(file_path).with_extension(format!("{}.new", file_extension));
        let readable_files = [UrlReadAction {
            url: full_input_path.as_path(),
            action: ReadAction::from_data("matching line\noutdated file content"),
        }];
        let writable_files = [UrlWriteAction {
            url: full_output_path.as_path(),
            action: WriteAction::Data,
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(readable_files);
        let mut writable_url_opener = WritableUrlOpener::new(writable_files);
        let file_content = "matching line\nfile content";

        let result = super::super::check_that_file_is_current_and_create_new_file(
            |path: &Path| readable_url_opener.open_url(path),
            |path: &Path| writable_url_opener.open_url(path),
            file_path,
            file_extension,
            file_content,
        );

        assert!(crate::test_result_equals_string(
            result,
            &format!(
                "{} is out of date and an updated version was saved, {}, to be used to overwrite it. The first line that did not match has index 1",
                full_input_path.display(),
                full_output_path.display()
            )
        ));
        assert!(readable_url_opener.is_url_reads_set_equal(&[full_input_path.as_path()]));
        assert!(writable_url_opener.is_url_writes_equal(&[UrlWrite {
            url: full_output_path.as_path(),
            outcome: UrlWriteContent::from_data(file_content),
        }]));
    }
}
