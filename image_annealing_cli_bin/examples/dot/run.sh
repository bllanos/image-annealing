#!/usr/bin/env bash
#
# One can achieve much better performance using the `image_annealing` library's API within a Rust program
# than using the CLI in an equivalent shell script.
#
# This script is intended to be a simple demonstration of the CLI, and is not written to be robust or fast.

BASE_OUTPUT_DIRECTORY="examples_output/image_annealing_cli_bin_dot"
SWAP_OUTPUT_DIRECTORY="${BASE_OUTPUT_DIRECTORY}/swap_permutations"
IMAGE_OUTPUT_DIRECTORY="${BASE_OUTPUT_DIRECTORY}/permuted_images"
rm -rf "${BASE_OUTPUT_DIRECTORY}"
mkdir -p "${BASE_OUTPUT_DIRECTORY}"
mkdir -p "${SWAP_OUTPUT_DIRECTORY}"
mkdir -p "${IMAGE_OUTPUT_DIRECTORY}"

cargo run -p image_annealing_cli_bin --example dot

cargo build -p image_annealing_cli_bin --release --bins

target/release/main image_annealing_cli_bin/examples/dot/config/create_permutation.json

target/release/main image_annealing_cli_bin/examples/dot/config/swap.json

INPUT_IMAGE_FILE="${BASE_OUTPUT_DIRECTORY}/image.png"
cp "${INPUT_IMAGE_FILE}" "${IMAGE_OUTPUT_DIRECTORY}/0.png"

PERMUTE_CONFIG_FILE="${BASE_OUTPUT_DIRECTORY}/permute_config.json"
i=1
ROUND=0
PASS=0
NUMBER_OF_FILES=$(find "${SWAP_OUTPUT_DIRECTORY}" -maxdepth 1 -type f -name '*.png' -printf x | wc -c)
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

    cat << _FILE_CONTENTS_ > "${PERMUTE_CONFIG_FILE}"
{
  "Permute": {
    "candidate_permutation": "${PERMUTATION_FILE}",
    "original_image": {
      "Rgba8": "${INPUT_IMAGE_FILE}"
    },
    "permuted_image_output_path_no_extension": {
      "Rgba8": "${IMAGE_OUTPUT_DIRECTORY}/${i}"
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