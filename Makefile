NAME=pog_api

all: build-release

.PHONY: build-release
build-release:
	cargo build --release

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	set -a && . ./config.env && ./target/debug/pog-api-rs

.PHONY: run-release
run-release:
	set -a && . ./config.env && ./target/debug/pog-api-rs

.PHONY: docker-build
docker-build:
	docker build -t $(NAME) .

.PHONY: docker-run
docker-run:
	docker run --rm --env-file config.env --name $(NAME) $(NAME):latest

.PHONY: docker-exec-shell
docker-exec-shell:
	docker exec -it $(NAME) /bin/bash

.PHONY: format-fix
format-fix:
	cargo fmt --all --

.PHONY: format-check
format-check:
	cargo fmt --all -- --check

.PHONY: scan
scan:
	cargo audit -D warnings --ignore RUSTSEC-2020-0159 --ignore RUSTSEC-2020-0071

.PHONY: lint
lint clean:
	cargo clippy --all-features -- -D warnings

.PHONY: check-deps
check-deps:
	cargo outdated --root-deps-only

.PHONY: check-license
check-license:
	cargo license

.PHONY: docs
docs:
	cargo doc --open

.PHONY: clean
clean:
	cargo clean
