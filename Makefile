.PHONY: dev dev-trace dev-all run check clippy test fmt help

DEV_LOG := commander_tournament=debug,commander_tournament_core=debug
TRACE_LOG := commander_tournament=trace,commander_tournament_core=trace

dev: ## Run the app with debug logging for workspace crates
	RUST_LOG=$(DEV_LOG) cargo run

dev-trace: ## Run the app with trace logging for workspace crates
	RUST_LOG=$(TRACE_LOG) cargo run

dev-all: ## Run the app with debug logging for all crates (including dependencies)
	RUST_LOG=debug cargo run

run: ## Run the app without extra RUST_LOG overrides
	cargo run

check: ## Check all workspace targets
	cargo check --workspace --all-targets

clippy: ## Run clippy across all workspace targets
	cargo clippy --workspace --all-targets

test: ## Run all workspace tests
	cargo test --workspace

fmt: ## Format all workspace crates
	cargo fmt --all

help: ## Show available make targets
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'
