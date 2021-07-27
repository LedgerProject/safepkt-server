SHELL:=/bin/bash

.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

install-deps: ## Install cargo
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

build: ## Build application
	@cargo build

release: ## Release application
	@cargo build --release

test: ## Test application
	@cargo test