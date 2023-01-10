SHELL:=/bin/bash

filepath := $(PWD)
versionfile := ./VERSION
version := $(shell cat $(versionfile))

check-docker-image-tag:
	echo "opentensorfdn/rinzler:$(version)"

build:
	cargo build --release

build-docker:
	docker build -f Dockerfile -t opentensorfdn/rinzler:$(version) .

push-docker:
	docker push opentensorfdn/rinzler:$(version)
