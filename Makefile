
build:
	maturin develop
	# cargo build

ui: build
	RUST_BACKTRACE=full cargo test --test ui_test

.PHONY: build ui