LD_LIBRARY_PATH := $(DESTDIR)/tmp/lib
PATH := $(DESTDIR)/tmp/bin:${PATH}

.EXPORT_ALL_VARIABLES:

default: all

all: build

build: rustup
	$(DESTDIR)/tmp/bin/cargo build --release --verbose

rustup:
	curl -L https://static.rust-lang.org/rustup.sh | bash -s -- --prefix=$(DESTDIR)/tmp/

test:
	$(DESTDIR)/tmp/bin/cargo test

install:
	mkdir -p $(DESTDIR)/usr/bin
	install -m 0755 target/release/rrun $(DESTDIR)/usr/bin/rrun

deb:
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-ignore-new --git-pbuilder
