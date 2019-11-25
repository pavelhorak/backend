all: src/*
	cargo build --release

dev: src/*
	cargo build
