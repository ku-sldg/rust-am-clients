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

am_client_cert_fixed:
	cargo run --release --bin rust-am-client -- -t testing/protocol_cert.json -c 127.0.0.1:5042

am_client_cert_appr:
	cargo run --release --bin rust-am-client -- -t testing/protocol_cert_appr.json -e testing/glob_type_env_cert_appr.json -g testing/glob_comps_cert_appr.json

am_client_cert_appr_fixed:
	cargo run --release --bin rust-am-client -- -t testing/protocol_cert_appr.json -e testing/glob_type_env_cert_appr.json -g testing/glob_comps_cert_appr.json -c 127.0.0.1:5043

am_client_micro:
	cargo run --release --bin rust-am-client -- -t testing/protocol_micro.json -e testing/glob_type_env_micro.json -g testing/glob_comps_micro.json

am_client_micro_session:
	cargo run --release --bin rust-am-client -- -t testing/protocol_micro.json -a testing/session_micro.json

am_client_cds_dynamic:
	cargo run --release --bin rust-am-client -- -t testing/cds_tpm.json 2>/dev/null

am_client_cds_static:
	cargo run --release --bin rust-am-client -- -t testing/cds_tpm_static.json 2>/dev/null

resolute_client_help:
	cargo run --release --bin rust-resolute-client -- --help

resolute_client_cert_appr:
	cargo run --release --bin rust-resolute-client -- -r testing/req_resolute_cert_appr.json -e testing/env_resolute_cert_appr.json

resolute_client_cert_appr_fixed:
	cargo run --release --bin rust-resolute-client -- -r testing/req_resolute_cert_appr.json -e testing/env_resolute_cert_appr.json -c 127.0.0.1:5044

resolute_client_micro:
	cargo run --release --bin rust-resolute-client -- -r testing/req_resolute_micro.json -e testing/env_resolute_micro.json

resolute_client_micro_fixed:
	cargo run --release --bin rust-resolute-client -- -r testing/req_resolute_micro.json -e testing/env_resolute_micro.json -c 127.0.0.1:5045

clean:
	rm -rf $(BIN)
	cargo clean
