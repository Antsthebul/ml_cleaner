run-app:
	cargo tauri dev

# Starts the DB/ Mock 3rd party API server
run-local:
	docker compose up -d

build-local:
	docker compose build

run-prd:
	docker compose up db -d

stop:
	docker compose down