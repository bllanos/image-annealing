use image_annealing::config;
use image_annealing::image_utils::rectangle::Rectangle;
use std::error::Error;

#[test]
fn bad_rect_negative() {
    let v = vec!["one", "two", "--rect", "-1", "-1", "1", "2"];
    let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
    let r = config::parse_args(v_strings);
    r.expect_err("`--rect` with negative numbers afterwards should be an error");
}

#[test]
fn bad_rect_malformed() {
    let v = vec!["one", "two", "--rect", "1", "2", "1", "0"];
    let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
    let r = config::parse_args(v_strings);
    r.expect_err("`--rect` impossible coordinates afterwards should be an error");
}

#[test]
fn bad_rect_empty() {
    let v = vec!["one", "two", "--rect", "1", "0", "2", "0"];
    let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
    let r = config::parse_args(v_strings);
    r.expect_err("`--rect` empty rectangle coordinates afterwards should be an error");
}

#[test]
fn good_rect() -> Result<(), Box<dyn Error>> {
    let v = vec!["one", "two", "--rect", "1", "0", "2", "5"];
    let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
    let r = config::parse_args(v_strings.clone())?;
    assert_eq!(r.0, v_strings[1]);
    assert_eq!(
        r.1.ok_or("Expected a valid rectangle")?,
        Rectangle::from_corners(1, 0, 2, 5)?
    );
    Ok(())
}
