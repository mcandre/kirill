.POSIX:
.SILENT:
.PHONY: \
	all \
	audit \
	build \
	cargo-check \
	clean \
	clean-archive \
	clean-cargo \
	clean-crit \
	clean-example \
	clean-packages \
	clean-ports \
	clippy \
	crit \
	doc \
	docker-build \
	docker-push \
	docker-test \
	install \
	lint \
	package \
	port \
	publish \
	rustfmt \
	test \
	uninstall \
	upload
.IGNORE: \
	clean \
	clean-archive \
	clean-cargo \
	clean-crit \
	clean-example \
	clean-packages \
	clean-ports

VERSION!=cargo metadata --format-version 1 --no-deps | jq -r ".packages[0].version"
BANNER=kirill

all: build

audit:
	cargo audit

build: lint test
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean-archive \
	clean-cargo \
	clean-crit \
	clean-example \
	clean-ports

clean-archive:
	rm .crit/bin/$(BANNER).tgz

clean-cargo:
	cargo clean

clean-crit:
	crit -c

clean-example:
	rm -f example/Cargo.lock
	rm -rf example/target
	rm -rf example/.crit

clean-packages:
	rm -rf .rockhopper

clean-ports:
	rm -rf .crit/bin/kirill-ports

clippy:
	cargo clippy

crit:
	crit -b $(BANNER)

doc:
	cargo doc

docker-build:
	docker buildx bake all

docker-push:
	docker buildx bake production --push

docker-test:
	docker buildx bake test --push

install:
	cargo install --force --path .

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt

package:
	rockhopper -r "version=$(VERSION)"

port:
	./port -C .crit/bin -a kirill $(BANNER)

publish:
	cargo publish

rustfmt:
	cargo fmt

test:
	cargo test

uninstall:
	cargo uninstall crit

upload:
	./upload
