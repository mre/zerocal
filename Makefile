.PHONY: run dev
run dev:
	cargo shuttle run

.PHONY: deploy
deploy:
	cargo shuttle deploy