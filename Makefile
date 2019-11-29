all: src/* frontend
	cargo build --release

dev: src/* frontend
	cargo build

docs: src/*
	cargo doc --no-deps --target-dir docs

frontend: frontend/
	cd frontend && make

docker: Dockerfile
	docker build -t cw .

.PHONY: docker frontend docs
