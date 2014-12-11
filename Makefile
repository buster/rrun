default: all

all: clean build

build: rustup
	cargo build --release --verbose

rustup:
	which cargo || curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --prefix=$(TMPDIR)

test:
	cargo test

install:
	mkdir -p $(DESTDIR)/usr/bin
	install -m 0755 target/release/rrun $(DESTDIR)/usr/bin/rrun

clean: rustup
	cargo clean
	git clean -f
