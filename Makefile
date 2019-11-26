all: src/* frontend
	cargo build --release

dev: src/* frontend
	cargo build

frontend: frontend/
	cd frontend
	npm i
	npm run build

docker: Dockerfile
	docker build -t cw .
