# List Tasks
default:
    just --list

# Runs the Development-Kit Container
dkc:
    docker pull ghcr.io/leoborai/dkc:latest
    docker run -it --rm \
        -v $(pwd):/app \
        -w /app \
        ghcr.io/leoborai/dkc:latest

# Dev build (unoptimized, fast compile) into ./dist
build:
    cargo run

# Release build (optimized) into ./dist
release:
    cargo run --release

# Dev mode: dev build, then serve ./dist at http://localhost:8080
dev: build
    python3 -m http.server 8080 --directory dist

# Release build, then serve ./dist at http://localhost:8080
serve: release
    python3 -m http.server 8080 --directory dist

# Formats Rust code
fmt:
    cargo fmt --all
