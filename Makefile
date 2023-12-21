.PHONY: test
test:
	cargo test

.PHONY: lint
lint:
	cargo clippy

.PHONY: lint/fix
lint/fix:
	cargo clippy --fix
