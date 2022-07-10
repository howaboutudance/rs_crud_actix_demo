APP_IMAGE_NAME := rs-crud-actix
CONT_EXEC := $(if $(shell command -v "podman"), podman, docker)

build: build-app

build-app:
	${CONT_EXEC} build -f Dockerfile --target app -t ${APP_IMAGE_NAME}

run: build reload
	${CONT_EXEC} run -p 8000:8000 ${APP_IMAGE_NAME}

env:
	./scripts/deps-up.sh

clean:
	./scripts/deps-dn.sh

reload: | clean env