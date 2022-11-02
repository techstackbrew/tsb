run:
	cargo run

build:
	cargo build --release

test: build
	./target/release/tsb
	./target/release/tsb brew upgrade
