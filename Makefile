SHELL:=/bin/bash

.PHONY: help build development-server lint start test

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

build: ## Build production package
	@/bin/bash -c '( test -e .env && source .env || true ) && npx nuxt-ts generate'

copy-configuration-file: ## Copy default configuration file
	cp .env{.dist,}

development-server: ## Start development server
	@/bin/bash -c 'source .env && npx nuxt-ts'

install-deps: ## Install JavaScript dependencies
	@npm install

lint: ## Lint project files
	@/bin/bash -c 'npm run lint'

test: ## Run tests
	@/bin/bash -c 'NODE_ENV=test npx jest'

start: ## Start production server
	@/bin/bash -c 'source .env && npx nuxt-ts start'
