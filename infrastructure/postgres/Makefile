POSTGRES_CONTAINER_NAME=postgres-db
DOCKERFILE_DIR=.
POSTGRES_PORT=5432
INIT_SQL=init.sql

# Start PostgreSQL container
start:
	docker run -d \
	  --name $(POSTGRES_CONTAINER_NAME) \
	  -e POSTGRES_USER=postgres \
	  -e POSTGRES_PASSWORD=password \
	  -e POSTGRES_DB=yelp \
	  -p $(POSTGRES_PORT):5432 \
	  -v $(DOCKERFILE_DIR)/$(INIT_SQL):/docker-entrypoint-initdb.d/$(INIT_SQL):ro \
	  postgis/postgis:latest

build:
	docker build -t yelp-postgres $(DOCKERFILE_DIR)

# Stop and clean up
stop:
	docker stop $(POSTGRES_CONTAINER_NAME) || true
	docker rm $(POSTGRES_CONTAINER_NAME) || true

# Rebuild and restart
restart: stop build start

# Clean everything
clean: stop
	docker rmi yelp-postgres || true
