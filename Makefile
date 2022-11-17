.ONESHELL:

buid:
	cd thebigg_dev && cargo build

run_container: build_image
	docker run -d -p 8080:8080 thebigg.dev:thebigg.dev 

build_image: 
	docker build . -t thebigg.dev:thebigg.dev

login:
	docker login
push_container: login build_image
	docker tag thebigg.dev:thebigg.dev thebigg1/thebigg.dev
	docker push thebigg1/thebigg.dev

