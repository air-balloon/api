.PHONY: build test format migrate-status

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
