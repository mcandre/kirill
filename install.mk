.POSIX:
.SILENT:
.PHONY: all

all:
	rustup component add \
		clippy \
		rustfmt
	cargo install --force \
		cargo-audit \
		cargo-cache \
		cargo-edit
