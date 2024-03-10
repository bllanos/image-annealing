//! This module is intended to contain all code that performs side-effects,
//! such as interacting with out-of-process dependencies.
//!
//! Mocks/spies of functions in this module are used in tests to verify that project
//! code correctly executes observable side-effects. Unobservable side-effects
//! are run as-is during integration tests, rather than being replaced with mocks.
//!
//! This approach to testing is described in
//! [Unit Testing Principles, Practices, and Patterns](https://www.manning.com/books/unit-testing),
//! by Vladimir Khorikov, Manning Publications, published January 2020 (ISBN 9781617296277).
//!
//! Do not write tests that test functions in this module.
//! All functions in this module should be simple, such that they can be verified by inspection.

pub use std::env::var_os as read_environment_variable;
