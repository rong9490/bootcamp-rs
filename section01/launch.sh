#! /bin/bash

# cargo run -- csv
# cargo run -- csv --format yaml --input assets/juventus.csv --output assets/juventus.yaml
cargo run -- genpass
# cargo run -- base64 encode --input Cargo.lock

# cargo run -- csv --format yaml --input crates/claps/assets/juventus.csv --output crates/claps/assets/juventus.json
# cargo run -- gpass --length 16 --uppercase --lowercase --number --symbol