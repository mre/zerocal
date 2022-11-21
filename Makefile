.PHONY: run dev
run dev: ## Run the app in development mode on shuttle
	cargo shuttle run

.PHONY: watch local
watch local: ## Run the app in local development mode
	cargo watch -x 'run --features local'

.PHONY: deploy
deploy: ## Deploy to shuttle
	cargo shuttle deploy

.PHONY: lint
lint: ## Run linter
	cargo clippy --all-targets --all-features -- -D warnings