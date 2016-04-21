.DEFAULT_GOAL := help

test: ## Run the test suite
	rm -f /tmp/filedes_fun/mysock* ; env RUST_BACKTRACE=1 cargo test

dockertest: ## Run the test suite in a docker container
	docker build -t current .
	docker run -ti current

doc: ## Build docs for this project and its dependencies
	cargo doc

## Self-documentation (http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html)
.PHONY: help
HELP_ALIGN_COLUMN := 10
help:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-${HELP_ALIGN_COLUMN}s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
