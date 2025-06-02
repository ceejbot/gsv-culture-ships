BINNAME := "codefact"
RELATIVE_TAP_PATH := "../../../homebrew-tap/"

_help:
	just -l

# Run all tests using nextest.
test:
	cargo nextest run

# Run the same checks we run in CI. Requires nightly.
ci: test
	cargo clippy
	cargo +nightly fmt

# Ask for clippy's opinion.
lint:
	cargo clippy --fix
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
	version=$(echo "$current" | semver-bump {{BUMP}})
	tomato set package.version "$version" Cargo.toml &> /dev/null
	cargo generate-lockfile
	git commit Cargo.toml -m "v${version}"
	git tag "v${version}"
	echo "Release tagged for version v${version}"

# Publish to crates.io.
release:
	cargo publish
