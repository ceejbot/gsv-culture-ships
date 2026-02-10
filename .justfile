BINNAME := "culture-ship"
RELATIVE_TAP_PATH := "../../../homebrew-tap/"

_help:
    just -l

# Run all tests using nextest.
test:
    cargo nextest run
    cargo nextest run --features noncanonical

# Run the same checks we run in CI. Requires nightly.
ci: test
    cargo clippy
    cargo clippy --features noncanonical
    cargo +nightly fmt --check

# Format and fix lints.
lint: fmt
    cargo clippy --fix

fmt:
    cargo +nightly fmt

# Install required tools
setup:
    brew tap ceejbot/tap
    brew install fzf tomato semver-bump cargo-nextest
    rustup install nightly

# Tag a new version for release.
version BUMP:
    #!/usr/bin/env bash
    set -e
    current=$(tomato get package.version Cargo.toml)
    version=$(semver-bump {{ BUMP }} "$current")
    tomato set package.version "$version" Cargo.toml &> /dev/null
    cargo generate-lockfile
    git commit Cargo.toml Cargo.lock -m "v${version}"
    git tag "v${version}"
    echo "Release tagged for version v${version}"

# Publish to crates.io.
release:
    cargo publish
