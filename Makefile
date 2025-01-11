run-app:
	cargo tauri dev

# Starts the DB/ Mock 3rd party API server
run-local:
	docker compose --profile live up -d

build-local:
	docker compose build

run-prd:
	docker compose up db -d

stop:
	docker compose down

setup-time-series:
	docker run -d -p 8086:8086 \
		-v ${PWD}/.influxdb/data:/var/lib/influxdb2 \
		-v ${PWD}/.influxdb/config:/etc/influxdb2 \
		-e DOCKER_INFLUXDB_INIT_MODE=setup \
		-e DOCKER_INFLUXDB_INIT_USERNAME=db_user \
		-e DOCKER_INFLUXDB_INIT_PASSWORD=db_user \
		-e DOCKER_INFLUXDB_INIT_ORG=my-org \
		-e DOCKER_INFLUXDB_INIT_BUCKET=my-bucket \
		-e DOCKER_INFLUXDB_INIT_ADMIN_TOKEN=my-super-secret-auth-token \
		-e V1_DB_NAME=v1-db \
		-e V1_AUTH_USERNAME=v1-user \
		-e V1_AUTH_PASSWORD=v1-password \		
		influxdb:2

# ATOW, --profile down stops ALL services
run-test:
	docker-compose --profile test up -d
	-cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture --test-threads 1
	docker-compose stop test-db

check:
	cargo check --manifest-path src-tauri/Cargo.toml