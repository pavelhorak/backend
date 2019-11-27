all: src/* frontend
	cargo build --release

dev: src/* frontend
	cargo build

frontend: frontend/
	cd frontend
	make

docker: Dockerfile
	docker build -t cw .
