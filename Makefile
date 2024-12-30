watch:
	cargo watch -x run
watch-check:
	cargo watch -x check
check:
	cargo check
test:
	cargo test
coverage:
	cargo tarpaulin --ignore-tests
lint:
	cargo clippy -- -D warnings
format:
	cargo fmt
audit:
	cargo audit