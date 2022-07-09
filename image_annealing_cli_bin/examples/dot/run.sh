#!/usr/bin/env bash
#
# One can achieve much better performance using the `image_annealing` library's API within a Rust program
# than using the CLI in an equivalent shell script.
#
# This script is intended to be a simple demonstration of the CLI, and is not written to be robust or fast.

IMAGE_WIDTH=200

# Stop the script on any error
set -e -u -o pipefail

BASE_OUTPUT_DIRECTORY="examples_output/image_annealing_cli_bin_dot"
SWAP_OUTPUT_DIRECTORY="${BASE_OUTPUT_DIRECTORY}/swap_permutations"
IMAGE_OUTPUT_DIRECTORY="${BASE_OUTPUT_DIRECTORY}/permuted_images"
rm -rf "${BASE_OUTPUT_DIRECTORY}"
mkdir -p "${BASE_OUTPUT_DIRECTORY}"
mkdir -p "${SWAP_OUTPUT_DIRECTORY}"
mkdir -p "${IMAGE_OUTPUT_DIRECTORY}"

cargo run -p image_annealing_cli_bin --release --example dot -- "${IMAGE_WIDTH}"

CREATE_PERMUTATION_CONFIG_FILE="${BASE_OUTPUT_DIRECTORY}/create_permutation_config.json"
cat << _FILE_CONTENTS_ > "${CREATE_PERMUTATION_CONFIG_FILE}"
{
  "CreatePermutation": {
    "image_width": ${IMAGE_WIDTH},
    "image_height": ${IMAGE_WIDTH},
    "permutation_output_path_no_extension": "${BASE_OUTPUT_DIRECTORY}/initial_permutation"
  }
}
_FILE_CONTENTS_

cargo build -p image_annealing_cli_bin --release --bins

target/release/main "${CREATE_PERMUTATION_CONFIG_FILE}"

target/release/main image_annealing_cli_bin/examples/dot/config/swap.json

INPUT_IMAGE_FILE="${BASE_OUTPUT_DIRECTORY}/image.png"

# Example of using [ImageMagick](https://imagemagick.org/) to make sure the input image has an alpha channel
# See https://imagemagick.org/script/command-line-options.php#alpha
# Uncomment and update the following lines if you want to use an input image file
# that may not have the expected format.
#
# RAW_INPUT_IMAGE_FILE="some_image.jpeg"
# convert -alpha opaque "${RAW_INPUT_IMAGE_FILE}" "${INPUT_IMAGE_FILE}"

NUMBER_OF_FILES=$(find "${SWAP_OUTPUT_DIRECTORY}" -maxdepth 1 -type f -name '*.png' -printf x | wc -c)
FIELD_WIDTH=${#NUMBER_OF_FILES}
PADDED_NUMBER="$(printf "%0${FIELD_WIDTH}d" "0")"
cp "${INPUT_IMAGE_FILE}" "${IMAGE_OUTPUT_DIRECTORY}/${PADDED_NUMBER}.png"

PERMUTE_CONFIG_FILE="${BASE_OUTPUT_DIRECTORY}/permute_config.json"
i=1
ROUND=0
PASS=0
while [ $i -le "${NUMBER_OF_FILES}" ]; do
    PERMUTATION_FILE="${SWAP_OUTPUT_DIRECTORY}/permutation_round_${ROUND}_pass_${PASS}"
    case $PASS in
        0) PERMUTATION_FILE="${PERMUTATION_FILE}_horizontal.png";;
        1) PERMUTATION_FILE="${PERMUTATION_FILE}_vertical.png";;
        2) PERMUTATION_FILE="${PERMUTATION_FILE}_offset_horizontal.png";;
        3) PERMUTATION_FILE="${PERMUTATION_FILE}_offset_vertical.png";;
        *)
            echo "invalid swap pass index"
            exit 1;;
    esac
    echo "Permuting with ${PERMUTATION_FILE}"

    PADDED_NUMBER="$(printf "%0${FIELD_WIDTH}d" "$i")"
    cat << _FILE_CONTENTS_ > "${PERMUTE_CONFIG_FILE}"
{
  "Permute": {
    "candidate_permutation": "${PERMUTATION_FILE}",
    "original_image": {
      "Rgba8": "${INPUT_IMAGE_FILE}"
    },
    "permuted_image_output_path_no_extension": {
      "Rgba8": "${IMAGE_OUTPUT_DIRECTORY}/${PADDED_NUMBER}"
    }
  }
}
_FILE_CONTENTS_

    target/release/main "${PERMUTE_CONFIG_FILE}"

    i=$(( i + 1 ))
    PASS=$(( PASS + 1 ))
    if [ "${PASS}" -ge 4 ]; then
        PASS=0
        ROUND=$(( ROUND + 1 ))
    fi
done

# Animated GIF generation requires [ImageMagick](https://imagemagick.org/) to be installed.
# See https://askubuntu.com/questions/43763/tool-to-convert-a-sequence-of-numbered-png-files-to-an-animated-gif
# Uncomment the following lines to generate an animation file

# ANIMATION_FILE="image_annealing_cli_bin/examples/dot/animation.gif"
# echo "Updating animation ${ANIMATION_FILE}..."
# convert -dispose none "${IMAGE_OUTPUT_DIRECTORY}/*.png" -set delay 1 -loop 0 "${ANIMATION_FILE}"