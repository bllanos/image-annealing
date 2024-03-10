<!-- omit in toc -->
# Development goals

<!-- omit in toc -->
## Table of Contents

- [Minimality](#minimality)
- [Learning](#learning)
- [Accessibility](#accessibility)
- [Quality](#quality)
- [Interface design](#interface-design)
  - [Side effects](#side-effects)
    - [Definition](#definition)
    - [Production code](#production-code)
    - [Test code](#test-code)
- [Non-goals](#non-goals)

The [README](../README.md#vision-and-future-development) describes the primary goals of the project. Some secondary goals are described below, as well as some associated development guidelines.

## Minimality

1. Minimize the number of direct library dependencies. Every dependency introduces development and maintenance overhead as it must be kept up to date and supported by facade or glue code, making the codebase larger.

2. Do not develop a domain-specific language.

3. Do not develop a general-purpose framework. Write code that supports a small set of algorithms. There are many existing frameworks and they do not need to be reinvented.

4. Delegate parallelization to external code. External code can choose whether to parallelize across threads, processes, and/or computers. Within the library there is therefore no performance overhead from synchronization mechanisms. This approach is loosely-inspired by [libvips](https://github.com/libvips/libvips/wiki/Why-is-libvips-quick).

   The obvious exception to this point is our decision to use GPU computing, which involves parallel computation. We chose to use GPU computing out of a desire to learn, and we chose to implement algorithms that are suitable for GPU processing. Also, to our knowledge, the choice of whether or not a library uses GPU computing cannot be delegated to external code without support from the library. Either a library must have code dedicated for different kinds of processors, or the library must be compiled to bytecode that a separate runtime can convert to code that executes on a dynamically-chosen processor (e.g. CPU or GPU).

5. Delegate non-essential functionality, such as support for a variety of input and output image file formats, to third-party programs. Follow the [Unix philosophy of doing one thing well](https://en.wikipedia.org/wiki/Unix_philosophy).

## Learning

1. Learn by developing the mechanisms that are hidden by frameworks, instead of using a framework. Examples of frameworks are systems for running code on either the CPU or the GPU without modification.

2. Encourage contribution to the project by developing the project using tools and libraries that people want to learn about. [Minimality](#minimality) helps in this respect by limiting the number of internal conventions and techniques that need to be learned in order to contribute, but that are irrelevant outside the context of the project.

## Accessibility

Accessibility means making the project easier to use and contribute to.

1. Program in conventional Rust. Avoid tools that make extensive use of macros or other techniques that modify code syntax or create domain-specific languages on top of Rust.

2. Use widespread communication protocols and data/file formats for input and output.

3. Use [WebGPU Shading Language](https://gpuweb.github.io/gpuweb/wgsl/) for GPU programming as it is host (CPU-side) programming language-agnostic and therefore reusable by developers from a variety of programming language communities.

4. More generally, make it easy for anyone who invests effort in using or contributing to the project to reuse the results of their efforts somewhere else.

## Quality

1. Strive to maintain an effective test suite and high code test coverage.

2. Recycle previously allocated memory and other resources between operations. Avoid creating short-lived objects that dynamically-allocate memory or perform other expensive resource management operations.

3. Use mutability (`mut`) as little as possible, without sacrificing performance. Mutability often makes the behavior of code harder to understand.

4. Assume that the user has more important programs to run on the same computer at the same time as programs from this project. Programs from this project should therefore minimize their use of memory and processing power in order to yield resources to other programs.

   This principle applies to command-line programs designed to be run by users. If we were implementing programs that serve other programs, we may have different priorities.

   We assume that users will run other programs concurrently because in general, the algorithms implemented in this project are iterative algorithms that will run for longer periods of time.

   Given two viable implementations, prefer the one that conserves resources even if it may take somewhat longer to execute. If a program runs faster but uses more resources, it may force the user to spend more time waiting, depending on whether there are sufficient resources remaining for the user to work on other tasks in the meantime.

   Here are some concrete examples:

   1. It is acceptable for command-line programs to block on input/output operations because blocking allows the computer to work on more important tasks. Parallelizing input/output operations introduces overhead that consumes more resources.
   2. Data transfer between the CPU and the GPU should be done in a blocking manner so as to eliminate [additional copies of data](https://github.com/gfx-rs/wgpu/discussions/1438).

5. Allow programs to process larger volumes of data by reducing the extra memory required per unit of input/output data. Minimizing copying, such as by avoiding [`clone()`](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html?#variables-and-data-interacting-with-clone), can help achieve this goal.

## Interface design

Strive to design flexible interfaces.

1. Design un-opinionated interfaces while striving to design interfaces that prevent users from making mistakes or triggering undefined or unexpected behavior.

2. All functionality in the codebase should be available through all mediums, where possible, including:

   1. The Rust interfaces of the code
   2. Command-line interfaces provided for the code

3. Client code should be allowed to directly modify any data it might reasonably want to change. Software layers that enable controlled modification of data are unlikely to please everyone and often require considerable effort to develop. They may be perceived as obstacles rather than assistants, so avoid creating them.

4. Expose interfaces with different levels of detail that allow for extensibility at each level:

   1. The command-line interface crate (`image_annealing_cli`) has a function for handling a bounded set of events, as events are modelled as a Rust `enum`.

   2. The high-level operations crate (`image_annealing_operation`) has an event handling function, and a separate function for handling each type of event. New functions can be added to handle new types of events. Internally, this crate submits GPU command buffers, but delegates GPU command buffer preparation to the lower-level crate described next. The lower-level crate is a private dependency.

   3. The [wgpu](https://wgpu.rs/)-based implementation crate (`image_annealing_compute`) exposes the individual operations and resources from `wgpu` that are needed to perform operations. In contrast, the higher-level crates do not expose `wgpu` as a public dependency, nor do they depend directly on `wgpu`, and they hide intermediate resources and function calls that are used to handle events.

      The `image_annealing_compute` crate's interface loosely-follows the [wgpu guidelines on encapsulating graphics work](https://github.com/gfx-rs/wgpu/wiki/Encapsulating-Graphics-Work).

5. Use free functions in preference to functions that belong to data types or traits. Think about whether any function (or part of a function) could be useful outside the context of the data type or trait to which it is associated.

### Side effects

#### Definition

The definition of a side-effect is somewhat vague. An [article on algebraic effects by Matija Pretnar (2015)](https://www.eff-lang.org/handlers-tutorial.pdf) inspired the approach we would like to take towards side effects, which is to encapsulate side-effects in injected services. For our purposes, the most important side-effects are:

1. Communication with out-of-process dependencies such as environment variables, files, and input and output streams

2. Mutation (`mut`)

3. Code that selects control flow patterns, such as how Rust `Future`s are run to completion, or how the process accommodates asynchronous GPU operations.

4. [Panicking](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html).

#### Production code

A command-line application's `run()` function should be the only code that performs side-effects. At least, this is the ideal, but we will need to be pragmatic at times.

The code that implements side-effects (other than mutation) should be as minimal as possible, and should be defined outside of `run()`. `run()` should be the only function that **directly or indirectly** invokes side-effect functions. In this project, we define side-effect execution code in [`image_annealing_cli_util/src/side_effect/mod.rs`](../image_annealing_cli_util/src/side_effect/mod.rs). Refer to the documentation of that module for guidelines on its use.

`run()` is the highest-level function within an application that can be called by a Rust integration test, meaning that the function [must be in a library crate](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests-for-binary-crates). It is the outermost function that encapsulates all the application logic, and is invoked by `main()`. `main()` and Rust integration tests pass side-effect services (or mock versions of services) to `run()`. `run()` therefore does not choose how side-effects are executed.

#### Test code

For test code, there is no need to limit the use of mutation (`mut`), but it is still best to limit other side-effects to keep tests fast and easier to maintain. In this project, we have written side-effect execution code for tests in [`test_util/src/side_effect/mod.rs`](../test_util/src/side_effect/mod.rs). Refer to the documentation of that module for guidelines on its use.

## Non-goals

The following goals are not part of the project vision.

1. Developing graphical user interfaces. There are too many possible requirements to satisfy, so if we develop graphical user interfaces, it will likely be in a different code repository.

2. Developing tools for working with multidimensional data having more than two dimensions.

   At present, we do not have a compelling use case for such tools. We will wait until there is a compelling use case, and would likely pursue development in a different code repository because the implementation may be highly dependent on the use case. In particular, the [curse of dimensionality](https://en.wikipedia.org/wiki/Curse_of_dimensionality) means that a simple representation of multidimensional data using a dense grid (which is the approach taken for two-dimensional data in this repository) is usually not the best choice. Depending on the use case, one of several different sparse representations may be more appropriate, such as [graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)) or [space-partitioning data structures](https://en.wikipedia.org/wiki/Space_partitioning#Data_structures).
