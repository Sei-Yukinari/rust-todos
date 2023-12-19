SHELL := /bin/bash

.DEFAULT_GOAL := help
.PHONY: help
help:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(RUN_ARGS):;@:)

up: ## Run Container
	@docker-compose -f ./docker-compose.yml up -d --build --force-recreate

down: ## Stop Container
	@docker-compose -f ./docker-compose.yml down

clean: ## Clean Container
	@docker-compose -f ./docker-compose.yml down --remove-orphans --volumes

ps: ## List Container
	@docker-compose -f ./docker-compose.yml ps

stop: ## Stop Container Selected Environment
	@docker-compose -f ./docker-compose.yml stop $(RUN_ARGS)

logs: ## Show Container log Selected Environment
	@docker-compose -f ./docker-compose.yml logs -f $(RUN_ARGS)

exec: ## Enter Container Selected Environment
	@docker-compose -f ./docker-compose.yml exec $(RUN_ARGS) bash

gen: ## Generate GraphQL Schema
	@docker-compose -f ./docker-compose.yml exec web cargo run --bin generate-schema

test: ## run unit test
	@docker-compose -f ./docker-compose.yml exec web cargo test