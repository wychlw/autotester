
build:
	maturin develop
	# cargo build

ui: build
	cargo test --test ui_test

.PHONY: build ui