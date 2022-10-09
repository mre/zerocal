.PHONY: run dev
run dev:
	cargo shuttle run

.PHONY: local
local:
	cargo watch -x 'run --features local'

.PHONY: deploy
deploy:
	cargo shuttle deploy