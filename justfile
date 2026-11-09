set shell := ["bash", "-euo", "pipefail", "-c"]

check:
    just --fmt --check
    cargo fmt -- --check
    cargo deny --locked check licenses sources
    cargo shear --locked --deny-warnings

# Execution host or runner only; provider-check remains separate.
test: build audit
    cargo clippy --locked --all-targets -- -D warnings
    cargo test --locked

provider-check:
    python3 scripts/check-provider.py

# List the project commands.
default:
    @just --list

# Fetch locked source dependencies without compiling.
setup:
    cargo fetch --locked

# Compile the locked release workspace.
build:
    cargo build --locked --workspace --release

# Check current RustSec advisories.
audit:
    cargo audit --deny warnings

# Run source checks, native tests and the advisory audit.
verify: check test audit build
