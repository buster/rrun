LD_LIBRARY_PATH := $(DESTDIR)/tmp/lib
PATH := $(DESTDIR)/tmp/bin:${PATH}

.EXPORT_ALL_VARIABLES:

default: all

all: clean build

build:
	cargo build --release --verbose

test:
	cargo test

install:
	mkdir -p $(DESTDIR)/usr/bin
	install -m 0755 target/release/rrun $(DESTDIR)/usr/bin/rrun

deb:
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-ignore-new --git-pbuilder

local-deb:
	debuild --preserve-env --prepend-path=/usr/local/bin -d binary

release:
	git-dch -a -c -R --full --debian-tag="v%(version)s" --git-author
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-pbuilder --git-tag --git-debian-tag="v%(version)s"

clean:
	rm -rf target

snapshot:
	git-dch -a -S --full --debian-tag="v%(version)s" --git-author
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-ignore-new --git-pbuilder  --git-debian-tag="v%(version)s"

updatecowbuilder:
	sudo cowbuilder --update

# How to install rust in pbuilder/cowbuilder without debian packages:
# sudo cowbuilder --login --save-after-login
# apt-get update
# apt-get dist-upgrade
# apt-get install rustc cargo libgtk-3-dev git sudo curl debhelper ca-certificates sed tar bash
# exit
