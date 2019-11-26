all: src/*
	cargo build --release

dev: src/*
	cargo build

docker: Dockerfile
	docker build -t cw .
