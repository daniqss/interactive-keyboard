.PHONY: build run

build: 
	 docker build -f app/Dockerfile -t interactive-keyboard-web app

run:
	docker compose up
