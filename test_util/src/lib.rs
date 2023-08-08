pub mod algorithm;
pub mod image;
pub mod operation;
pub mod path;
pub mod permutation;
pub mod shader;

pub fn assert_error_contains<T: std::fmt::Debug, E: std::fmt::Display>(
    result: Result<T, E>,
    message: &str,
) {
    let e = result.unwrap_err();
    if !e.to_string().contains(message) {
        eprintln!("error message: {}", e);
        eprintln!("expected substring: {}", message);
        panic!("error message does not contain the expected substring")
    }
}
