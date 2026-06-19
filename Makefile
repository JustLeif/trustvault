ENCLAVE_PORT ?= 5000

dev:
	$(MAKE) -j2 enclave-dev host-dev

enclave-dev:
	ENCLAVE_PORT=$(ENCLAVE_PORT) cargo watch -x "run -p enclave"

host-dev:
	ENCLAVE_PORT=$(ENCLAVE_PORT) cargo watch -x "run -p host"
