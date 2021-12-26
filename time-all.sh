#!/bin/bash

hyperfine --warmup 2 'cargo test --release --quiet --lib -- --test-threads=1 regression'
