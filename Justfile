default:
    just --list

dev:
    cd ./crates/www && trunk serve --config ./Trunk.toml

fmt:
  cargo clippy --workspace --fix --allow-dirty --allow-staged && cargo fmt
  leptosfmt crates/www/**/*.rs

index-notes:
  cargo run --bin cli -- notes
