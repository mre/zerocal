.PHONY: run dev
run dev:
	cargo shuttle run

.PHONY: watch local
watch local:
	cargo watch -x 'run --features local'

.PHONY: deploy
deploy:
	cargo shuttle deploy