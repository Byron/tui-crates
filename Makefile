.PHONY : tests build

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Testing

tests: ## Run all tests we have to assert quality
	cd crosstermion && make check
	cd tui-react && cargo check

tests-windows: ## Run all tests that have a chance to work on windows
	cd crosstermion && make check-windows
	cd tui-react && cargo check

##@ Maintenance
clean: ## cargo clean on all crates
	cargo clean

