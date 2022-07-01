#!/usr/bin/env bash

cargo run -p image_annealing_cli_bin --example dot

cargo run -p image_annealing_cli_bin -- image_annealing_cli_bin/examples/dot/config/create_permutation.json

cargo run -p image_annealing_cli_bin -- image_annealing_cli_bin/examples/dot/config/swap.json