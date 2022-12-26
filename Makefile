.ONESHELL:

check_format:
	rustfmt --check thebigg_dev/src/*.rs

format:
	rustfmt  thebigg_dev/src/*.rs

build:
	cd thebigg_dev && cargo build

nuke_images:
	docker system prune -a -f --volumes

run_container: build_image
	docker run -d -p 8080:8080 thebigg.dev:thebigg.dev 

build_image: 
	docker build . -t thebigg.dev:thebigg.dev

login:
	docker login
push_container: login build_image
	docker tag thebigg.dev:thebigg.dev thebigg1/thebigg.dev
	docker push thebigg1/thebigg.dev

#Very useful to deploy on server
run_app_nginx:
	docker pull thebigg1/thebigg.dev
	docker rm -f thebigg_dev
	service nginx restart
	docker run --name thebigg_dev -d -p 8080:8080 thebigg1/thebigg.dev:latest
