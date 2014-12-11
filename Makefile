default: all

all: clean build

build: rustup
	/usr/local/bin/cargo build --release --verbose

rustup:
	curl -s https://static.rust-lang.org/rustup.sh | sudo sh

test:
	/usr/local/bin/cargo test

install:
	install -m 0755 target/release/rrun $(prefix)/bin

clean: rustup
	/usr/local/bin/cargo clean
	git clean -f
