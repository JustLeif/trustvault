.PHONY: dev

ENCLAVE_PORT ?= 5000
HOST_PORT ?= 50051

dev:
	@set -e; \
	trap 'kill 0' INT TERM EXIT; \
	ENCLAVE_PORT=$(ENCLAVE_PORT) cargo watch -x "run -p enclave" & \
	ENCLAVE_PID=$$!; \
	ENCLAVE_PORT=$(ENCLAVE_PORT) HOST_PORT=$(HOST_PORT) cargo watch -x "run -p host" & \
	HOST_PID=$$!; \
	npm run dev:web & \
	WEB_PID=$$!; \
	wait $$ENCLAVE_PID $$HOST_PID $$WEB_PID
