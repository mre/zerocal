.PHONY: run dev
run dev:
	cargo shuttle run

.PHONY: local
local:
	cargo run --features local

.PHONY: deploy
deploy:
	cargo shuttle deploy