local:
	cargo watch -c -w ./src -- cargo run --bin web_quick -- features dev
dev:
	cargo watch -c -w ./src -- cargo run --bin web_quick -- features dev

main:
	cargo watch -c -w ./src -- cargo run --bin web_quick -- features

test:
	cargo test

build:
	cargo build --release

clean:
	cargo clean