//! This module is intended to contain all code that performs side-effects during tests.
//!
//! Tests that interact directly with out-of-process resources can therefore be identified
//! by their use of items from this module. These tests can be audited in the future
//! when searching for tests that are possibly slow or hard to maintain,
//! and therefore might be removed or refactored.
//!
//! Do not use items from this module in test helper functions.
//! Helper functions should instead accept side-effect handlers as arguments.
//! Only test functions (functions annotated with `#[test]`) should reference items from this module.
//! All functions in this module should be simple, such that they can be verified by inspection.

use std::fs::File;
use std::path::Path;

pub fn open_file(path: &Path) -> std::io::Result<impl std::io::Read> {
    File::open(path)
}

pub fn create_file(path: &Path) -> std::io::Result<impl std::io::Write> {
    File::create(path)
}
