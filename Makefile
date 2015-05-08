LD_LIBRARY_PATH := $(DESTDIR)/tmp/lib
PATH := $(DESTDIR)/tmp/bin:${PATH}

.EXPORT_ALL_VARIABLES:

default: all

all: clean build

build: rustup
	$(DESTDIR)/tmp/bin/cargo build --release --verbose

rustup:
	curl -L https://static.rust-lang.org/rustup.sh | bash -s -- --prefix=$(DESTDIR)/tmp/ --disable-sudo

test:
	$(DESTDIR)/tmp/bin/cargo test

install:
	mkdir -p $(DESTDIR)/usr/bin
	install -m 0755 target/release/rrun $(DESTDIR)/usr/bin/rrun

deb:
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-ignore-new --git-pbuilder

release:
	git-dch -a -c -R --full 
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-pbuilder --git-tag --git-debian-tag="v%(version)s"

clean:
	rm -rf target

snapshot:
	git-dch -a -S --full
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-ignore-new --git-pbuilder  --git-debian-tag="v%(version)s"


