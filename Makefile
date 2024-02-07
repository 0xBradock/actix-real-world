DB_USER := postgres
DB_PASSWORD := password
DB_NAME := arw
DB_PORT := 5432

postgres-up:
	docker run -d --name ${DB_NAME} -p ${DB_PORT}:${DB_PORT} -e POSTGRES_PASSWORD=${DB_PASSWORD} -e POSTGRES_USER=${DB_USER} postgres

postgres-down:
	docker stop ${DB_NAME}

run:
	cargo run