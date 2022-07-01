#!/usr/bin/env bash
#
# One can achieve much better performance using the `image_annealing` library's API within a Rust program
# than using the CLI in an equivalent shell script.
#
# This script is intended to be a simple demonstration of the CLI, and is not written to be robust or fast.

cargo run -p image_annealing_cli_bin --example dot

cargo run -p image_annealing_cli_bin -- image_annealing_cli_bin/examples/dot/config/create_permutation.json

cargo run -p image_annealing_cli_bin -- image_annealing_cli_bin/examples/dot/config/swap.json

INPUT_IMAGE_FILE="examples_output/image_annealing_cli_bin_dot_image.png"
IMAGE_OUTPUT_DIRECTORY="examples_output/image_annealing_cli_bin_dot_permuted_images"
mkdir -p "${IMAGE_OUTPUT_DIRECTORY}"
cp "${INPUT_IMAGE_FILE}" "${IMAGE_OUTPUT_DIRECTORY}/0.png"

PERMUTE_CONFIG_FILE="examples_output/image_annealing_cli_bin_dot_permute_config.json"
i=1
ROUND=0
PASS=0
NUMBER_OF_FILES=$(find examples_output -maxdepth 1 -type f -name 'image_annealing_cli_bin_dot_permutation*' -printf x | wc -c)
while [ $i -le "${NUMBER_OF_FILES}" ]; do
    PERMUTATION_FILE="examples_output/image_annealing_cli_bin_dot_permutation_round_${ROUND}_pass_${PASS}"
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
      "Rgba8": "${IMAGE_OUTPUT_DIRECTORY}/${i}.png"
    }
  }
}
_FILE_CONTENTS_

    cargo run -p image_annealing_cli_bin -- "${PERMUTE_CONFIG_FILE}"

    i=$(( i + 1 ))
    PASS=$(( PASS + 1 ))
    if [ "${PASS}" -ge 4 ]; then
        PASS=0
        ROUND=$(( ROUND + 1 ))
    fi
done