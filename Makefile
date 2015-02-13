export LD_LIBRARY_PATH+=$(DESTDIR)/tmp/lib
export PATH+=$(DESTDIR)/tmp/bin

default: all

all: build

build: rustup
	cargo build --release --verbose

rustup:
	curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --prefix=$(DESTDIR)/tmp

test:
	cargo test

install:
	mkdir -p $(DESTDIR)/usr/bin
	install -m 0755 target/release/rrun $(DESTDIR)/usr/bin/rrun

#clean: 
#	git clean -f
