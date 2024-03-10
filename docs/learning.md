<!-- omit in toc -->
# Development guidelines and lessons learned

<!-- omit in toc -->
## Table of Contents

- [Lessons learned](#lessons-learned)
  - [Interface design](#interface-design)
  - [Newtypes](#newtypes)
    - [Use cases for newtypes](#use-cases-for-newtypes)
    - [Cases where newtypes should not be used](#cases-where-newtypes-should-not-be-used)
  - [Generic programming](#generic-programming)
  - [Error handling](#error-handling)
  - [Testing](#testing)
- [Resources](#resources)
  - [Core Rust programming language resources](#core-rust-programming-language-resources)
  - [Other Rust programming language resources](#other-rust-programming-language-resources)
  - [WebGPU](#webgpu)
  - [Software design](#software-design)

## Lessons learned

We developed the following guidelines through experience. We have not seen some of the guidelines in third-party references, or at least not in the same words.

### Interface design

1. Expose the most primitive operations that can be performed atomically.
2. Higher-level operations should be provided through separate interfaces that reuse the interfaces that provide primitive operations. For performance reasons, higher-level operations are not atomic, because atomicity requires creating temporary data alongside the current state in order to discard the temporary data if the operation cannot be completed.
3. Interfaces that follow the principle of [reverse communication](https://link.springer.com/chapter/10.1007/978-1-4614-4469-5_4) should be preferred. Our understanding of reverse communication is that a library does not run any client code that makes decisions concerning the next action to be taken by the library. All decision-making logic should be executed outside the library. Reverse communication simplifies the interface between the library and the client application because the library's interface does not need to model possible decisions made by the client application.
4. Prefer "library"-style interfaces over "framework"-style interfaces. Interfaces should be like helper functions that support client code instead of like facades that manage client code internally.

### Newtypes

The newtype pattern is explained in [The Rust Programming Language](https://doc.rust-lang.org/book/ch19-04-advanced-types.html#using-the-newtype-pattern-for-type-safety-and-abstraction). We learned that some newtypes are not helpful.

#### Use cases for newtypes

1. The type is needed to maintain invariants. Normally this means it is a return type of a function in the library.

2. If a function accepts a parameter of a given type, it should be designed to accept any possible value of that type. In other words, [functions should be total functions][using-types-effectively].

   It is preferable to use a newtype for a parameter that restricts the possible values the parameter can have than to allow the parameter to have a type with values that cannot be handled by the function. In the latter case, the function must return an error for certain values of the parameter, and would be a partial function, not a total function.

   On the other hand, sometimes the need to construct a newtype makes client code more verbose. There is a tradeoff between readability and strictness.

3. The type represents a computation that has been performed and therefore does not need to be repeated wherever the type is used.

#### Cases where newtypes should not be used

1. The newtype would ensure data matches a certain structure that can be expressed [directly using the type system][using-types-effectively] rather than implicitly using data validation functions.

   For example, instead of having a type like:

   ```rust
   // The string must contain spaces to separate the parts of the name,
   // and we need a constructor to enforce this convention.
   struct PersonFullName(String);
   ```

   use

   ```rust
   struct PersonPartOfName(String); // A string containing only letters

   // Represents the possibility of spaces between parts of the name using types
   struct PersonFullName {
     parts_of_name: Vec<PersonPartOfName>
   }
   ```

2. The newtype represents statements that may not be facts. For example, do not create a newtype that represents a file that is known to exist. The file could cease to exist during the lifetime of the newtype value.

3. Multiple newtypes would represent data that all have the same interface (i.e. behavior during operations) but that are not interchangeable, yet the newtypes are private and any accidental interchange of the data would cause unit tests to fail. If the tests would need to be written regardless of whether or not newtypes are used for the data, then the newtypes do not provide any benefit.

4. The newtype would only serve as a semantic label and is not required for correctness. For example, do not create separate newtypes like `FirstImage` and `SecondImage` if both images can have any possible contents and if their order ("first" vs. "second") is a extrinsic property, not an intrinsic property. In this example, one could use a type such as `ImagePair(Image, Image)` instead.

### Generic programming

1. Generic parameters should be treated like regular function data parameters. If client code would not be interested in passing different types for a generic parameter, the parameter should be removed, just like one would remove data parameters that client code does not care about. Generic parameters can otherwise leak implementation details.

### Error handling

1. Avoid using [`Box<dyn Error>`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html). Instead, define concrete error types that allow errors to be inspected programmatically, for the following reasons:

   1. `dyn Error` allows client code to obtain a string representation of the error determined by the underlying error type, but this representation has been chosen by the library defining the error type. Client code may prefer to create a different representation of the error to display to the user.

   2. `dyn Error` often hides the nature or cause of the error and therefore prevents client code from making informed decisions about how to handle the error.

2. Raise errors at the earliest time that it is certain an error has or will occur. When interacting with external systems, such as a database or a filesystem, do not check for possible errors in advance of a fallible operation when the state of the external system can change after the checks. In such circumstances, the code will have a [time-of-check to time-of-use](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use) vulnerability, unless error handling code is duplicated by repeating it after the fallible operation is performed. Code duplication is usually wasteful.

   For example, do not check if a file exists as part of command-line argument validation, and then try to access the file later.

3. Minimize the set of conditions that are considered errors. Error conditions that are based on assumptions/opinions concerning how the code should be used are normally detrimental to users. Such error conditions should preferably be removed after modifying the code so that the assumptions/opinions are no longer present. Errors should only be raised when there are problems that cannot be resolved.

   For example, if an image is provided as input and its dimensions do not match the dimensions of a buffer that would store it, reallocate the buffer instead of returning an error. Assume the caller is aware of the size mismatch and understands that the program will need to reallocate the buffer.

   This guideline supports [making functions total functions][using-types-effectively].

### Testing

1. Implement tests as Rust [unit tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests), even if they have the semantics of integration tests. Use Rust [integration tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests) only for tests that cannot be run in parallel with other tests.

2. Prefer only writing expressions that can panic inside tests, not inside production code. Panicking is a form of [effect](vision.md#side-effects), and also makes code harder to test, because tests of panic cases must use the [`should_panic`](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic) attribute.

   1. An assertion (e.g. `assert!()`) outside of test code is usually one of the following:

      1. A statement of a precondition of a function: Refactoring the function can sometimes convert it into a [total function][using-types-effectively] that does not have the precondition.

      2. An internal consistency check inside a function: Splitting the function into smaller functions may allow tests to be written that are equivalent to the former consistency check. Sometimes refactoring can eliminate the need for any consistency check.

         If it is certain that a consistency check will never fail, remove it. Tests can increase one's confidence that a consistency check will never fail.

      3. A statement of a postcondition of a function: Add tests that verify the postcondition, and remove the postcondition check from the function's code.

   2. Other expressions that can panic are usually one of the following:

      1. Cases that should never panic in practice.

         If the panic expression is inside a conditional statement, it is equivalent to an assertion and should be handled as described above. For example:

         ```rust
         if input.size == 0 {
             panic!();
         }
         ```

         If the panic expression is inside a third-party function and there is sufficient information to determine that the function should never panic, this is acceptable. For example:

         ```rust
         let value = std::num::NonZeroUsize::new(0).unwrap();
         ```

         Such cases can often be rewritten using unsafe code, for example:

         ```rust
         let value = unsafe { std::num::NonZeroUsize::new_unchecked(0) };
         ```

         We suggest avoiding unsafe code except where it provides significant performance benefits, as future code maintenance and refactoring can sometimes violate preconditions that are required to prevent undefined behavior.

      2. Cases that may occur in practice, meaning that panicking is being used as an error handling technique. [Use `Result` in preference to panicking](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html).

      3. `match` statement arms that include `unreachable!()`. Try to use a control flow structure other than `match`, or try to narrow the type of value that the `match` statement is operating on in order to eliminate the unreachable arm.

3. Within tests, only place expressions expressions that can panic in test functions (functions annotated with `#[test]`), not in test helper functions. Doing so makes the assertion portion of tests more obvious and makes it easier to write tests for test helper functions as needed.

4. Write tests for test helper functions (such as functions in the [`test_util`](../test_util/) crate), but only to test error cases. The intention of tests of test helper functions is to verify that test helper functions can detect deviations from expected behavior such that tests of production code will fail when production code deviates from expected behavior. Do not write tests for the success cases within test helper functions, otherwise code test coverage reports will not be able to identify unused test helper functions. Success cases should already be covered by tests of production code.

## Resources

We found the following resources helpful. Each one is listed alongside the date that it was most heavily used. Be mindful that, if a date is far in the past, then the resource may no longer exist or may have changed substantially since. The list is not exhaustive.

### Core Rust programming language resources

1. (2020) [The Rust Programming Language](https://doc.rust-lang.org/book/)
2. (2022) [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
3. (2021) [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
4. (2021) [The rustc book](https://doc.rust-lang.org/rustc/index.html)
5. (2021) [The Rust Command Line Book](https://rust-cli.github.io/book/index.html)

### Other Rust programming language resources

1. (2021) [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
2. (2021) [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
3. (2021) [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
4. (2021) [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
5. (2022) [Pretty State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/)

### WebGPU

1. (2020) [Learn Wgpu](https://sotrh.github.io/learn-wgpu/)
2. (2020) [wgpu wiki](https://github.com/gfx-rs/wgpu/wiki)
3. (2021) [WebGPU Shading Language](https://gpuweb.github.io/gpuweb/wgsl/)

### Software design

1. (2023) [Unit Testing Principles, Practices, and Patterns](https://www.manning.com/books/unit-testing), by  Vladimir Khorikov, Manning Publications, published January 2020 (ISBN 9781617296277)
2. (2023) [The Grug Brained Developer](https://grugbrain.dev/), probably written by Carson Gross
3. (2023) [Worse Is Better](https://www.dreamsongs.com/WorseIsBetter.html), by Richard P. Gabriel

[using-types-effectively]: https://elbeno.com/presentations/using-types-effectively/presentation.html
