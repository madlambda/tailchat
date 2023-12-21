.PHONY: test
test:
	cargo test

.PHONY: lint
lint:
	cargo fmt --check
	cargo clippy

.PHONY: lint/fix
lint/fix:
	cargo clippy --fix

.PHONY: fmt
fmt:
	cargo fmt
