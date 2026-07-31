set dotenv-load
set positional-arguments

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

# Perform formatting and linting
fmt:
    bun run format
