<!-- omit in toc -->
# 2D Permutation Toolbox

Tools for developing algorithms that work with permutations of data in 2D grids (such as images).

<!-- omit in toc -->
## Example

TODO The example will be recreated.

<!-- ![White dot on a black background being transformed into a black star on a white background](./image_annealing_cli_bin/examples/dot/animation.gif) -->

<!-- Instructions for running the example are in [`image_annealing_cli_bin/examples/dot/README.md`](./image_annealing_cli_bin/examples/dot/README.md). Before running the example, follow the [repository setup instructions](#setup) below. -->

<!-- omit in toc -->
## Table of Contents

- [Problem description](#problem-description)
  - [Description using an analogy](#description-using-an-analogy)
  - [Abstract description](#abstract-description)
  - [Why permute images?](#why-permute-images)
    - [Advantages of permutations](#advantages-of-permutations)
    - [Disadvantages of permutations](#disadvantages-of-permutations)
- [Setup](#setup)
  - [Supported platforms](#supported-platforms)
  - [Getting started](#getting-started)
- [Documentation](#documentation)
  - [Command-line interfaces](#command-line-interfaces)
- [Troubleshooting](#troubleshooting)
  - [Code panics with errors such as BadDisplay, BadContext, or NotInitialized](#code-panics-with-errors-such-as-baddisplay-badcontext-or-notinitialized)
- [Usage overview](#usage-overview)
  - [Data types](#data-types)
    - [Map](#map)
      - [Landing maps](#landing-maps)
      - [Launch maps](#launch-maps)
      - [Permutations](#permutations)
    - [Data](#data)
  - [Operations](#operations)
    - [Creation operations](#creation-operations)
    - [Swap](#swap)
    - [Map data](#map-data)
- [Vision and future development](#vision-and-future-development)
  - [Planned development](#planned-development)
- [Contributing](#contributing)
- [Contact](#contact)
- [License](#license)

## Problem description

### Description using an analogy

Suppose there is a crowd of people in a room. You want to determine the path that each person takes over time. Each person may have a destination they want to reach, or may want to be closer to specific people, or to people matching a certain description. In brief, the criteria that determine how people move are arbitrary.

### Abstract description

Find a permutation of data arranged in a 2D grid by swapping data elements in an attempt to decrease the value of some cost function that evaluates permutations. There is no guarantee that you will find a permutation corresponding to a global minimum of the cost function, but you will at least obtain a sequence of intermediate permutations that relates the initial and final arrangements of the data. You can choose the swap pattern so that, for example, each element moves at most one unit of distance between intermediate permutations, providing a continuous path for each element.

### Why permute images?

Images can be modelled as functions mapping pixel coordinates to colors. We think most image processing algorithms focus on the output of the mapping, modifying the colors. There are overlooked opportunities to explore operations on the input of the mapping, the coordinates. We want to experiment with rearranging the structures of images.

#### Advantages of permutations

[Permutations](https://en.wikipedia.org/wiki/Permutation) have some nice properties:

- Permutations are linear operators
- Permutations are invertible
- Permutations can be applied to sequences of any kind of data, not only to sequences of numbers
- Permuting sequences of numbers does not result in round-off (quantization) error
- Permutations preserve cardinality (area, in the case of digital images)
- Any permutation can be decomposed into a sequence of permutations wherein each element is either stationary or trades places with its nearest neighbors
- The motions of elements according to a permutation can be evaluated in parallel

#### Disadvantages of permutations

Finding a permutation that satisfies a given set of criteria is usually a hard combinatorial optimization problem. The [traveling salesperson problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem) is an example of such a problem.

In this project, we use [simulated annealing](https://mathworld.wolfram.com/SimulatedAnnealing.html) to optimize permutations. Simulated annealing is easy to parallelize because each trade can be evaluated using only local information.

## Setup

This project is written in [Rust](https://www.rust-lang.org/), and uses the [wgpu][wgpu] library to parallelize image processing algorithms on the GPU. GPU shader programs are currently written in [WGSL (WebGPU Shading Language)](https://gpuweb.github.io/gpuweb/wgsl/).

We also provide command-line programs (CLI) so that you do not need to work with Rust code directly, but you still need Rust development tools to build the programs.

### Supported platforms

The code may work on all platforms that [wgpu][wgpu] supports, but has been developed on Linux, primarily targeting Vulkan, and occasionally targeting OpenGL. [WebAssembly](https://webassembly.org/) support has not yet been explored, and would require additional development.

### Getting started

1. [Install Rust](https://www.rust-lang.org/learn/get-started)
2. Try running the [example](#example). The script that runs the example will also build the code.

## Documentation

### Command-line interfaces

- The main command-line program, which manipulates images, is documented in [image_annealing_cli.md](docs/image_annealing_cli.md).

## Troubleshooting

### Code panics with errors such as BadDisplay, BadContext, or NotInitialized

You may need to tell [wgpu][wgpu] to use a particular graphics backend. Refer to [wgpu's documentation](https://docs.rs/wgpu/latest/wgpu/struct.Backends.html) for a list of backends.

<!-- TODO Try changing `wgpu::Backends::all()` in [`device.rs`](./image_annealing/src/compute/device.rs) to the backend you wish to use. In the future, we may allow the backend to be set by an easier method, such as an environment variable, for example. -->

## Usage overview

In this section, we give a high-level overview of how to use the code by explaining key data types and operations.

### Data types

The [operations](#operations) currently implemented manipulate the types of data described below.

#### Map

A map is a 2D grid of 2D vectors describing how elements located in the cells of the grid move to other cells in the grid.

In memory, a map is an array of pairs of 16-bit [big-endian][endianness] [two's complement integers](https://en.wikipedia.org/wiki/Two%27s_complement), stored in [row-major order][row_major_order] such that it represents a 2D array.

The first and second integer in a pair represent the `x` and `y` components, respectively, of a displacement vector associated with the position of the pair in the grid.

- Vectors with positive `x`-components point to the right.
- Vectors with positive `y`-components point downwards.

The coordinates of cells in a map follow image [texture coordinate conventions](https://gpuweb.github.io/gpuweb/#coordinate-systems), except that integer values are used for coordinates, not fractions between zero and one. If the grid has `m` rows and `n` columns (i.e. its height is `m` and its width is `n`), then the coordinates of the top left, top right, bottom left, and bottom right cells are `(0, 0)`, `(n - 1, 0)`, `(0, m - 1)`, and `(n - 1, m - 1)`, respectively.

Maps are interpreted as being either of the following:

##### Landing maps

A landing map is a map where each cell stores a vector pointing towards the source cell whose element will be moved into the current cell.

For example, if the cell with coordinates `(2, 3)` stores the vector `(1, -2)`, then the map will move the element at coordinates `(3, 1)` to coordinates `(2, 3)`.

##### Launch maps

A launch map is a map that represents where each element wants to move. A key operation in the code modifies a landing map so that it better approximates a launch map. Using this operation (the [swap operation](#swap)), you can trade the hard problem of finding a permutation for the easier problem of specifying a launch map.

Each cell in a launch map stores a vector pointing towards the goal location.

For example, if the cell with coordinates `(2, 3)` stores the vector `(1, -2)`, then the element in this cell wants to move to the cell with coordinates `(3, 1)`.

Note that a launch map can be created from a landing map by inverting the landing map, **if** the landing map is invertible. For example, if the cell with coordinates `(2, 3)` in a landing map stores the vector `(1, -2)`, then in the corresponding launch map, the cell with coordinates `(3, 1)` stores the vector `(-1, 2)`.

Vectors in launch maps point towards destinations, whereas vectors in landing maps point towards sources.

##### Permutations

Permutations are not (yet) explicitly represented as a data type in the code. A permutation is a map that is invertible (i.e. a [bijection](https://en.wikipedia.org/wiki/Bijection)). This means that:

1. All vectors point to cells inside the boundaries of the map
2. No two vectors point to the same cell

Another way of defining a permutation is by construction:

1. The identity map, where each cell stores the vector `(0, 0)`, is a permutation.

2. A map that rearranges the elements of a grid to change the dimensions of the grid without changing the number of cells in the grid is a permutation. Such a map can be constructed using a row-major scheme for ordering cells as follows:

   Given:

   - Initial dimensions m1 rows by n1 columns
   - Final dimensions m2 rows by n2 columns

   then the cell with coordinates `(x, y)` in the map stores the vector `(mod(y * n2 + x, n1) - x, floor((y * n2 + x) / n1) - y)`, where `mod()` is the modulus function and `floor()` is the floor function.

   Proof:

   1. A cell with coordinates `(x, y)` in the reshaped grid has a row-major index of `k = y * n2 + x`.
   2. The cell with row-major index `k` in the original grid has coordinates `(mod(k, n1), floor(k / n1))`.
   3. The vector from the coordinates in the reshaped grid to the coordinates in the original grid, for the cell with row-major index `k` is `(mod(k, n1) - x, floor(k / n1) - y) = (mod(y * n2 + x, n1) - x, floor((y * n2 + x) / n1) - y)`.

3. Any map generated from a permutation by a [swap operation](#swap) is itself a permutation.

#### Data

"Data" means data elements in the grids that are moved around by [maps](#map). As this project is usually used to manipulate images, we have chosen to interpret data as 16-bit depth four-channel images.

In memory, data is an array of quadruplets of 16-bit [big-endian][endianness] values, stored in [row-major order][row_major_order] such that it represents a 2D array.

### Operations

This section describes the operations in the code at a high level (omitting some details).

#### Creation operations

There are operations for creating values of all kinds of [data types](#data-types), including identity [maps](#map).

#### Swap

Input:

- An input [launch map](#launch-maps) or [landing map](#landing-maps)
- A reference map of the other type ([landing map](#landing-maps) or [launch map](#launch-maps), respectively)
- A swap pattern to evaluate
- A swap cost threshold that determines whether a given swap of two elements will be accepted

The default swap cost function results in an output map that is more similar to the reference map. The swap operation produces the output map by swapping elements of the input map (without modifying the input map). If requested, the operation can also output statistics concerning the number of swaps that were accepted and the swap costs.

A swap pattern is expressed as a vector defining the relative position of the element to swap with the current element. Any vector can be used, but there are four common swap patterns:

1. `Right`: Swaps elements at even `x` coordinates with their neighbors to the right (swap vector `(1, 0)`)
2. `Up`: Swaps elements at even `y` coordinates with their neighbors above (swap vector `(0, -1)`)
3. `Left`: Swaps elements at even `x` coordinates with their neighbors to the left (swap vector `(-1, 0)`)
4. `Down`: Swaps elements at even `y` coordinates with their neighbors below (swap vector `(0, 1)`)

#### Map data

The map operation takes a [landing map](#landing-maps) and [data](#data). It outputs data that is the result of moving the input data elements according to the displacement vectors in the map.

There is no operation for applying a [launch map](#launch-maps) to data because a launch map may map multiple elements to the same location. We do not know of a way to resolve such collisions that will satisfy the requirements of all use cases.

## Vision and future development

We hope to build a set of programmatic interfaces and command-line tools that help users experiment with 2D permutations and with approximate optimization algorithms that operate on permutations. Users can use the code to run systematic experiments, and can incorporate the data and algorithms that result from their experiments into other works, such as graphical user interfaces.

See [vision.md](docs/vision.md) for more information about the project's goals.

<!-- TODO Update this section -->
### Planned development

The following is a list of tasks that we hope to complete in the future, time-permitting. It is not an exhaustive list. The order of the items does not necessarily indicate the order in which they may be completed.

- Allow the [swap operation](#swap) to use custom swap cost functions
- Implement operations that will generate data directly on the GPU that would otherwise need to be generated by client code:
  - Launch map generation
  - (Image) data generation
- Add more documentation
- Write more detailed guidelines and tips for contributing to the project
- Leverage more tools, libraries, and frameworks to improve the code, development processes, and collaboration

## Contributing

You are welcome to contribute to the repository. We hope to add more documentation and instructions to make contributing easier, and have some initial guidelines in [CONTRIBUTING.md](CONTRIBUTING.md). You can also [contact us](#contact) to discuss contribution ideas.

## Contact

Feel free to [open an issue](https://github.com/bllanos/image-annealing/issues) or [start a discussion](https://github.com/bllanos/image-annealing/discussions).

If you want to start a conversation outside of GitHub, you can contact us first by email, and then we can find the best place to continue the conversation. (See commit metadata for email addresses)

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[endianness]: https://en.wikipedia.org/wiki/Endianness
[row_major_order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
[wgpu]: https://wgpu.rs/
