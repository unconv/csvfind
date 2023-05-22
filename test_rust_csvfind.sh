#!/bin/bash

# Compile csvfind.rs
rustc -O -o rust-csvfind csvfind.rs

# Compile csvfind.rs
rustc -O -o rust-csvfind_v2 csvfind_v2.rs

# Wait for compilation to complete (adjust the duration as needed)
#sleep 1

# Test csvfind.rs
echo "Testing csvfind.rs"
./rust-csvfind products.csv sunglasses name > output1.txt

# Compare output1 with expected file
if cmp -s expected_output.txt output1.txt; then
  echo "Test passed: Output matches expected result for csvfind.rs"
  rm -rf output1.txt
else
  echo "Test failed: Output does not match expected result for csvfind.rs "
fi

# Test csvfind_v2.rs
echo "Testing csvfind_v2.rs"
./rust-csvfind products.csv sunglasses name > output2.txt

# Compare output2 with expected file
if cmp -s expected_output.txt output2.txt; then
  echo "Test passed: Output matches expected result for csvfind_v2.rs"
  rm -rf output2.txt
else
  echo "Test failed: Output does not match expected result for csvfind_v2.rs "
fi