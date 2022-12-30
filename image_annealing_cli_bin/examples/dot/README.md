<!-- omit in toc -->
# Command-line interface usage example

This example uses the command-line interface (CLI) of the toolbox to create a sequence of images shown in the animation below.

![White dot on a black background being transformed into a black star on a white background](./animation.gif)

## Prerequisites

The example is a [Bash](https://www.gnu.org/software/bash/) script, [`run.sh`](./run.sh). If your environment does not include Bash, and you do not wish to install Bash, you can probably still run most of the important commands in the script without rewriting them, such as the `cargo` commands. You are welcome to [contribute](../../../README.md#contributing) scripts for shells other than Bash.

## Running the example

1. Before running the example, follow the [project-wide setup instructions](../../../README.md#setup).
2. Open a shell session in the root directory of this repository.
3. From your shell, execute [`run.sh`](./run.sh) by running the following command:

   ```bash
   ./image_annealing_cli_bin/examples/dot/run.sh
   ```

4. The script will generate output files in `examples_output/image_annealing_cli_bin_dot` (a new directory that the script creates under the [`../../../examples_output/`](../../../examples_output) directory). You should see a sequence of images in `examples_output/image_annealing_cli_bin_dot/permuted_images` that matches the animation shown above.

## Generating an animated GIF

If you have [ImageMagick](https://imagemagick.org/) installed, you can uncomment the lines at the bottom of [`run.sh`](./run.sh) that use ImageMagick to generate the animated GIF shown above. As presently written, the script will overwrite `examples_output/image_annealing_cli_bin_dot/animation.gif`.

## What is happening in the animation?

In this example, the CLI is given a [displacement goal](../../../README.md#displacement-goals) wherein:

- Pixels in a disc around the center of the image want to move as far as possible away from the center of the image.
- Pixels outside the disk want to move to the center of the image.

The script starts with an identity [permutation](../../../README.md#permutations) and runs the [swap operation](../../../README.md#swap) many times to produce a sequence of permutations. The permutations evolve from the identity permutation to a permutation that better satisfies the displacement goals of all pixels.

After generating the permutations, the script applies the permutations to an image. In the image, pixels that want to move away from the image center are colored white, whereas pixels that want to move towards the image center are colored black. The result is a sequence of images that shows how the pixels move.

## Experiment

Modify the image generation function (`white_dot()` in [`synthesis/mod.rs`](./synthesis/mod.rs)) so that it produces an image with a larger range of colors, or modify [`run.sh`](./run.sh) so that it uses an arbitrary image as input to the permute operation. Look at the patterns that result from pixel swapping.
