default: all

all: clean build

build: rustup
	cargo build --release --verbose

rustup:
	which cargo || curl -s https://static.rust-lang.org/rustup.sh | sudo sh -s --

test:
	cargo test

install:
	install -m 0755 target/release/rrun $(prefix)/bin

clean: rustup
	cargo clean
	git clean -f
