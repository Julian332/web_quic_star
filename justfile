local:
    cargo watch -c -q -w ./src -- cargo run  --bin web_quick -F dev

dev:
    cargo watch -c -q -w ./src -- cargo run --bin web_quick -F dev

prod:
    cargo watch -c -q -w ./src -- cargo run --bin web_quick

test:
    cargo test

build:
    cargo build --release

clean:
    cargo clean
