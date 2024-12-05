run:
	cargo tauri dev

run-local:
	docker compose up -d

build-local:
	docker compose build

run-prd:
	docker compose up db -d

stop:
	docker compose down