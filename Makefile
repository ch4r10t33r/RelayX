.DEFAULT_GOAL := help

# Cargo features for builds.
FEATURES ?=

# Cargo profile for builds.
PROFILE ?= release

# Extra flags for Cargo.
CARGO_INSTALL_EXTRA_FLAGS ?=

CARGO_TARGET_DIR ?= target

.PHONY: build
build: # Build the RelayX binary into `target` directory.
	cargo build --bin relayx --features "$(FEATURES)" --profile "$(PROFILE)"

.PHONY: lint lint-fix install-tools

# Install required tools
install-tools:
	cargo install cargo-sort
	cargo install cargo-clippy
	cargo install cargo-fmt

# Run all linters
lint: 	
	cargo +nightly fmt --all
	cargo clippy --all --all-targets --features "$(FEATURES)" --no-deps -- --deny warnings

	# cargo sort
	cargo sort --grouped --workspace