BIN := target

# list of targets to not build by default
#DEFAULT_EXCLUDES ?= sig_tpm sig_tpm_appr
#COMPUTED_EXCLUDES = $(foreach exclude,$(DEFAULT_EXCLUDES),--exclude $(exclude))

default:
	cargo build --release --workspace #$(COMPUTED_EXCLUDES)

all: 
	cargo build --release

debug: 
	cargo build 

test:
	make default
	cargo test --workspace #$(COMPUTED_EXCLUDES)

am_client_help:
	cargo run --release --bin rust-am-client -- --help

am_client:
	cargo run --release --bin rust-am-client

am_client_cert:
	cargo run --release --bin rust-am-client -- -t testing/protocol_cert.json

am_client_cert_appr:
	cargo run --release --bin rust-am-client -- -t testing/protocol_cert_appr.json -e testing/glob_type_env_cert_appr.json -g testing/glob_comps_cert_appr.json

am_client_micro:
	cargo run --release --bin rust-am-client -- -t testing/protocol_micro.json -e testing/glob_type_env_micro.json -g testing/glob_comps_micro.json

resolute_client_help:
	cargo run --release --bin rust-resolute-client -- --help

resolute_client_cert_appr:
	cargo run --release --bin rust-resolute-client -- -r testing/req_resolute_cert_appr.json -e testing/env_resolute_cert_appr.json

resolute_client_micro:
	cargo run --release --bin rust-resolute-client -- -r testing/req_resolute_micro.json -e testing/env_resolute_micro.json

clean:
	rm -rf $(BIN)
	cargo clean
