test: ## Run the test suite
	rm -f /tmp/filedes_fun/mysock* ; env RUST_BACKTRACE=1 cargo test

doc: ## Build docs for this project and its dependencies
	cargo doc

.DEFAULT_GOAL := help

.PHONY: help

HELP_ALIGN_COLUMN := 10

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-${HELP_ALIGN_COLUMN}s\033[0m %s\n", $$1, $$2}'
