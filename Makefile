.ONESHELL:

buid:
	cd thebigg_dev && cargo build

run_container: build_image
	docker run -d -p 8080:8080 thebigg.dev:thebigg.dev 

build_image: 
	@docker build . -t thebigg.dev:thebigg.dev
