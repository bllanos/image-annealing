mod check_that_file_is_current_and_create_new_file {
    use crate::mock::url::{
        ReadableUrlOpener, UrlContent, UrlWithContent, WritableUrl, WritableUrlAction,
        WritableUrlOpener,
    };
    use std::borrow::Cow;
    use std::path::Path;

    type UrlWriteRecord<'a> = crate::mock::url::UrlWriteRecord<UrlWithContent<'a, Path>, &'a Path>;

    #[test]
    fn file_is_missing_is_an_error() {
        let mut readable_url_opener = ReadableUrlOpener::new(&[]);
        let mut writable_url_opener = WritableUrlOpener::<'_, Path>::new(&[]);
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
        assert!(crate::test_result_equals_string(result, &format!("failed to open file to check for up-to-date content at path {}: URL {:?} is not in the set of mock readable URLs", full_path.display(), full_path)));
        assert_eq!(readable_url_opener.url_read_records(), &[full_path]);
        assert!(writable_url_opener.is_url_write_records_empty());
    }

    #[test]
    fn failure_to_read_from_file_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_path = Path::new(file_path).with_extension(file_extension);
        let readable_files = [UrlWithContent {
            url: Cow::Borrowed(full_path.as_path()),
            content: UrlContent::IoError,
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(&readable_files);
        let mut writable_url_opener = WritableUrlOpener::<'_, Path>::new(&[]);
        let file_content = "file content";

        let result = super::super::check_that_file_is_current_and_create_new_file(
            |path: &Path| readable_url_opener.open_url(path),
            |path: &Path| writable_url_opener.open_url(path),
            file_path,
            file_extension,
            file_content,
        );

        assert!(crate::test_result_equals_string(result, &format!("failed to read line 0 from file to check for up-to-date content at path {}: error reading from mock url {:?}", full_path.display(), full_path)));
        assert_eq!(
            readable_url_opener.url_read_records(),
            &[full_path.as_path()]
        );
        assert!(writable_url_opener.is_url_write_records_empty());
    }

    #[test]
    fn failure_to_open_updated_file_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_input_path = Path::new(file_path).with_extension(file_extension);
        let full_output_path =
            Path::new(file_path).with_extension(format!("{}.new", file_extension));
        let readable_files = [UrlWithContent {
            url: Cow::Borrowed(full_input_path.as_path()),
            content: UrlContent::String(Cow::Borrowed("outdated file content")),
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(&readable_files);
        let mut writable_url_opener = WritableUrlOpener::<'_, Path>::new(&[]);
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
                "failed to open an updated file at path {}: URL {:?} is not in the set of mock writable URLs",
                full_output_path.display(), full_output_path
            )
        ));
        assert_eq!(
            readable_url_opener.url_read_records(),
            &[full_input_path.as_path()]
        );
        assert_eq!(
            writable_url_opener.url_write_records(),
            &[UrlWriteRecord::Inaccessible(full_output_path.as_path())]
        );
    }

    #[test]
    fn failure_to_write_to_file_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_input_path = Path::new(file_path).with_extension(file_extension);
        let full_output_path =
            Path::new(file_path).with_extension(format!("{}.new", file_extension));
        let readable_files = [UrlWithContent {
            url: Cow::Borrowed(full_input_path.as_path()),
            content: UrlContent::String(Cow::Borrowed("outdated file content")),
        }];
        let writable_files = [WritableUrl {
            url: Cow::Borrowed(full_output_path.as_path()),
            action: WritableUrlAction::IoError,
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(&readable_files);
        let mut writable_url_opener = WritableUrlOpener::new(&writable_files);
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
                "failed to write to an updated file at path {}: error writing to mock url {:?}",
                full_output_path.display(),
                full_output_path
            )
        ));
        assert_eq!(
            readable_url_opener.url_read_records(),
            &[full_input_path.as_path()]
        );
        assert_eq!(
            writable_url_opener.url_write_records(),
            &[UrlWriteRecord::WriteResult(UrlWithContent {
                url: Cow::Borrowed(full_output_path.as_path()),
                content: UrlContent::IoError,
            })]
        );
    }

    #[test]
    fn outdated_file_contents_on_first_line_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_input_path = Path::new(file_path).with_extension(file_extension);
        let full_output_path =
            Path::new(file_path).with_extension(format!("{}.new", file_extension));
        let readable_files = [UrlWithContent {
            url: Cow::Borrowed(full_input_path.as_path()),
            content: UrlContent::String(Cow::Borrowed("outdated file content\nmatching line")),
        }];
        let writable_files = [WritableUrl {
            url: Cow::Borrowed(full_output_path.as_path()),
            action: WritableUrlAction::Data,
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(&readable_files);
        let mut writable_url_opener = WritableUrlOpener::new(&writable_files);
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
        assert_eq!(
            readable_url_opener.url_read_records(),
            &[full_input_path.as_path()]
        );
        assert_eq!(
            writable_url_opener.url_write_records(),
            &[UrlWriteRecord::WriteResult(UrlWithContent {
                url: Cow::Borrowed(full_output_path.as_path()),
                content: UrlContent::String(Cow::Borrowed(file_content)),
            })]
        );
    }

    #[test]
    fn outdated_file_contents_on_second_line_is_an_error() {
        let file_path = "file path";
        let file_extension = "test";
        let full_input_path = Path::new(file_path).with_extension(file_extension);
        let full_output_path =
            Path::new(file_path).with_extension(format!("{}.new", file_extension));
        let readable_files = [UrlWithContent {
            url: Cow::Borrowed(full_input_path.as_path()),
            content: UrlContent::String(Cow::Borrowed("matching line\noutdated file content")),
        }];
        let writable_files = [WritableUrl {
            url: Cow::Borrowed(full_output_path.as_path()),
            action: WritableUrlAction::Data,
        }];
        let mut readable_url_opener = ReadableUrlOpener::new(&readable_files);
        let mut writable_url_opener = WritableUrlOpener::new(&writable_files);
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
        assert_eq!(
            readable_url_opener.url_read_records(),
            &[full_input_path.as_path()]
        );
        assert_eq!(
            writable_url_opener.url_write_records(),
            &[UrlWriteRecord::WriteResult(UrlWithContent {
                url: Cow::Borrowed(full_output_path.as_path()),
                content: UrlContent::String(Cow::Borrowed(file_content)),
            })]
        );
    }
}
