run:
	cargo install --path . && dodo $(command) 
watch:
	cargo watch -x run
watch-check:
	cargo watch -x check
check:
	cargo check
test:
	TEST_LOG=true cargo test | bunyan
coverage:
	cargo tarpaulin --ignore-tests
lint:
	cargo clippy -- -D warnings
format:
	cargo fmt
audit:
	cargo audit