## Vars

TEST_CMDLINE := cargo test --
ifeq (1,${TESTBT})
TEST_CMDLINE_PREFIX := env RUST_BACKTRACE=1
else
TEST_CMDLINE_PREFIX := env
endif

## Targets

.DEFAULT_GOAL := help

test: ## Run the test suite
	bash -c "ulimit -n 500 ; ${TEST_CMDLINE_PREFIX} ${TEST_CMDLINE} ${TESTOPT}"

dockertest: ## Run the test suite in a docker container
	docker build -t current .
	docker run -ti current make -C /src test TESTBT="${TESTBT}" TESTOPT="${TESTOPT}"

doc: ## Build docs for this project and its dependencies
	cargo doc

## Self-documentation (http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html)
.PHONY: help
HELP_ALIGN_COLUMN := 10
help:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-${HELP_ALIGN_COLUMN}s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
