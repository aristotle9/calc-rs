#!/bin/bash
cargo build --release
cc -o test.out test/test.c target/release/libcalc.a
./test.out