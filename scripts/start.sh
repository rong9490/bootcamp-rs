#! /bin/bash


echo "cargo is runhning..."

# cargo run -- csv --format yaml --input crates/claps/assets/juventus.csv --output crates/claps/assets/juventus.json
# cargo run -- gpass --length 16 --uppercase --lowercase --number --symbol
cargo run -- base64 encode --input Cargo.lock