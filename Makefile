.PHONY: fmt check test clippy ci demo update-api

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace

test:
	cargo test --workspace --all-targets

clippy:
	cargo clippy -p memos-desktop --all-targets --no-deps -- -D warnings

ci: fmt check test clippy

demo:
	cargo run -p memos-desktop -- --demo

update-api:
	scripts/update-api.sh
