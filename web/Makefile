NAME=oncall_bot_web
VERSION=latest
TAG=registry.rdelfin.com/${NAME}:${VERSION}

.PHONY: build
build:
	docker build . -t ${TAG}

.PHONY: publish
publish:
	docker push ${TAG}
