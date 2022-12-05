#!/usr/bin/env sh
~/projects/padmini/explain.py --code $1 --pada $2
echo "\n\n~~~~~\n\n"
cargo run --bin explain -- --code $1 --pada $3
