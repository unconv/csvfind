#!/usr/bin/env bash
echo "Compiling...\n";

gcc -O -o c-csvfind-test-orig csvfind_orig.c
gcc -O -o c-csvfind-test csvfind.c
rustc -C opt-level=3 -C lto -C codegen-units=1 -o rust-csvfind-test csvfind.rs
rustc -C opt-level=3 -C lto -C codegen-units=1 -o rust-csvfindv2-test csvfind_v2.rs
rustc -C opt-level=3 -C lto -C codegen-units=1 -o rust-csvfindv3-test csvfind_v3.rs

echo "Testing...\n";

hyperfine --warmup 3 "php csvfind.php output.csv winkpad" \
"php csvfind_v2.php output.csv winkpad" \
"php -dopcache.enable_cli=1 -dopcache.jit_buffer_size=100M -dopcache.jit=1255 csvfind_v2.php output.csv winkpad" \
"./c-csvfind-test-orig output.csv winkpad" \
"./c-csvfind-test output.csv winkpad" \
"./rust-csvfind-test output.csv winkpad" \
"./rust-csvfindv2-test output.csv winkpad" \
"./rust-csvfindv3-test output.csv winkpad"

rm c-csvfind-test-orig
rm c-csvfind-test
rm rust-csvfind-test
rm rust-csvfindv2-test
rm rust-csvfindv3-test

