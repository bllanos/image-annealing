<!-- omit in toc -->
# Development goals

<!-- omit in toc -->
## Table of Contents

- [Minimality](#minimality)
- [Learning](#learning)
- [Accessibility](#accessibility)
- [Quality](#quality)
- [Interface design](#interface-design)
- [Non-goals](#non-goals)

The [README](../README.md#vision-and-future-development) describes the primary goals of the project. Some secondary goals are described below.

## Minimality

1. Minimize the number of direct library dependencies. Every dependency introduces development and maintenance overhead as it must be kept up to date and supported by facade or glue code, making the codebase larger.

2. Do not develop a domain-specific language.

3. Do not develop a general-purpose framework. Write code that supports a small set of algorithms. There are many existing frameworks and they do not need to be reinvented.

4. Delegate parallelization to external code. External code can choose whether to parallelize across threads, processes, and/or computers. Within the library there is therefore no performance overhead from synchronization mechanisms. This approach is loosely-inspired by [libvips](https://github.com/libvips/libvips/wiki/Why-is-libvips-quick).

   The obvious exception to this point is our decision to use GPU computing, which involves parallel computation. We chose to use GPU computing out of a desire to learn, and we chose to implement algorithms that are suitable for GPU processing.

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

3. Use mutability (`mut`) as little as possible, as it makes the behaviour of code easier to understand, but avoid sacrificing performance.

4. Assume that the user has more important programs to run on the same computer at the same time as programs from this project. Programs from this project should therefore minimize their use of memory and processing power in order to yield resources to other programs.

   This principle applies to command-line programs designed to be run by users. If we were implementing programs that serve other programs, we may have different priorities.

   We assume that users will run other programs concurrently because in general, the algorithms implemented in this project are iterative algorithms that will run for longer periods of time.

   Given two viable implementations, prefer the one that conserves resources even if it may take somewhat longer to execute. If a program runs faster but uses more resources, it may force the user to spend more time waiting depending on whether there are sufficient resources remaining for the user to work on other tasks in the meantime.

   Here are some concrete examples:

   1. It is acceptable for command-line programs to block on input/output operations because blocking allows the computer to work on more important tasks. Parallelizing input/output operations introduces overhead that consumes more resources.
   2. Data transfer between the CPU and the GPU should be done in a blocking manner so as to eliminate [additional copies of data](https://github.com/gfx-rs/wgpu/discussions/1438).

5. Allow programs to handle larger volumes of data by reducing the extra memory required per unit of input/output data.

## Interface design

Strive to design flexible interfaces.

1. Design un-opinionated interfaces while striving to limit mistakes and to prevent undefined or unexpected behaviour.

2. All functionality in the codebase should be available through all mediums, where possible, including:

   1. The Rust interfaces of the code
   2. Command-line interfaces provided for the code

3. Client code should be allowed to directly modify any data it might reasonably want to change. Software layers that enable controlled modification of data are unlikely to please everyone and often require considerable effort to develop. They may be perceived as obstacles rather than assistants, so avoid creating them.

4. Expose interfaces with different levels of detail that allow for extensibility at each level:

   1. The command-line interface crate (`image_annealing_cli`) has a function for handling a bounded set of events, as events are modelled as a Rust `enum`.

   2. The high-level operations crate (`image_annealing_operation`) has an event handling function, and a separate function for handling each type of event. New functions can be added to handle new types of events. Internally, this crate submits GPU command buffers, but delegates GPU command buffer preparation to the lower-level crate described next. The lower-level crate is a private dependency.

   3. The [wgpu](https://wgpu.rs/)-based implementation crate (`image_annealing_compute`) exposes the individual operations and resources from `wgpu` that are needed to perform operations. In contrast, the higher-level crates do not expose `wgpu` as a public dependency, nor do they depend directly on `wgpu`, and they hide intermediate resources and function calls that are used to handle events.

   The implementation crate's interface loosely-follows the [wgpu guidelines on encapsulating graphics work](https://github.com/gfx-rs/wgpu/wiki/Encapsulating-Graphics-Work).

5. Use free functions in preference to functions that belong to data types or traits. Think about whether any function (or part of a function) could be useful outside the context of the data type or trait to which it is associated.

6. A command-line application's `run()` function should be the only code that performs [side-effects](https://www.eff-lang.org/handlers-tutorial.pdf). At least, this is the ideal, but we will need to be pragmatic at times. Side-effects include:

   1. Communication with out-of-process dependencies such as environment variables, files, and input and output streams

   2. Mutation (`mut`)

   3. Imposing specific execution patterns, such as how Rust `Future`s are run to completion, or how the process handles asynchronous GPU operations.

## Non-goals

The following goals are not part of the project vision.

1. Developing graphical user interfaces. There are too many possible requirements to satisfy, so if we develop graphical user interfaces, it will likely be in a different code repository.

2. Developing tools for working with multidimensional data having more than two dimensions.

   At present, we do not have a compelling use case for such tools. We will wait until there is a use case that warrants developing such tools, and would likely develop them in a different code repository because the implementation may be highly dependent on the use case. In particular, the [curse of dimensionality](https://en.wikipedia.org/wiki/Curse_of_dimensionality) means that a simple representation of multidimensional data using a dense grid (which is the approach taken for two-dimensional data in this repository) is usually not the best choice. Depending on the use case, one of several different sparse representations may be more appropriate, such as [graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)) or [space-partitioning data structures](https://en.wikipedia.org/wiki/Space_partitioning#Data_structures).
