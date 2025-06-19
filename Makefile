define USAGE
Available commands:

    help       display this help
    test       run tests
endef

export USAGE

help:
	@echo "$$USAGE"

build:
	cargo build --no-default-features
	cargo build --all-features

clippy:
	cargo clippy -- -W "clippy::pedantic" -A "clippy::missing_errors_doc"

test:
	@test -x "$(which cargo-llvm-cov)" \
	&& (set -v; cargo test -- --test-threads=1) \
	|| (set -v; cargo llvm-cov test --html --output-dir=coverage -- --test-threads=1)

.PHONY: help build clippy test
