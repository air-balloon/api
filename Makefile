.PHONY: build build-docker run-docker test format migrate-status

DOCKER_IMAGE ?= air-balloon

CONTAINER_NAME ?= air-balloon-api
CONTAINER_NETWORK ?= local_williamdeslocal
CONTAINER_BIND_PORT ?= 8081
# Internal port, keep it the same as CONTAINER_BIND_PORT or you will get confused
ROCKET_PORT ?= 8081

migrate:
	@diesel migration run

migrate-status:
	@diesel migration list

build:
	@cargo build

test:
	@cargo test

format:
	@cargo fmt -- --emit files

build-docker:
	@docker build -t $(DOCKER_IMAGE) -f docker/Dockerfile ./

run-docker:
	@docker kill $(CONTAINER_NAME) || echo 'skip kill'
	@docker run --name $(CONTAINER_NAME) --network $(CONTAINER_NETWORK) -t --rm --env-file ./.env -p $(CONTAINER_BIND_PORT):$(ROCKET_PORT) -e ROCKET_PORT=$(ROCKET_PORT) $(DOCKER_IMAGE)
