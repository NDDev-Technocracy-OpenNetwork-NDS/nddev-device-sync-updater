set shell := ["bash", "-euo", "pipefail", "-c"]

check:
    cargo fmt -- --check
    cargo deny --locked check licenses sources
    cargo shear --locked --deny-warnings

# Execution host or runner only; provider-check remains separate.
test:
    cargo clippy --locked --all-targets -- -D warnings
    cargo test --locked

provider-check:
    python3 scripts/check-provider.py
