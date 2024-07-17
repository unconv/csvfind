#!/bin/bash

# Compile the specified Rust script
rustc -O -o rust-script "$RUST_TEST_FILE"

# Execute the test
./rust-script products.csv sunglasses name > output.txt

# Compare output with expected file
if cmp -s expected_output.txt output.txt; then
  echo "Test passed: Output matches expected result for $RUST_TEST_FILE"
  rm -rf output.txt
else
  echo "Test failed: Output does not match expected result for $RUST_TEST_FILE"
  cat output.txt
  exit 1
fi
