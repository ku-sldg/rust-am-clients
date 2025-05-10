BIN := target

# Variables used in make targets to configure various clients
PROTOCOLS_DIR=testing/protocols/
GLOBS_DIR=testing/globals/
SESSIONS_DIR=testing/attestation_sessions/
RODEO_REQUESTS_DIR=testing/rodeo_requests/
RODEO_ENVS_DIR=testing/rodeo_envs/

default:
	cargo build --release --workspace

all: 
	cargo build --release

debug: 
	cargo build 

test:
	make default
	cargo test --workspace

am_client_help:
	cargo run --release --bin rust-am-client -- --help

am_client:
	cargo run --release --bin rust-am-client

am_client_cert:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert.json

am_client_cert_fixed:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert.json -c 127.0.0.1:5042

am_client_cert_appr:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert_appr.json -e $(GLOBS_DIR)glob_type_env_cert_appr.json -g $(GLOBS_DIR)glob_comps_cert_appr.json

am_client_cert_appr_fixed:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert_appr.json -e $(GLOBS_DIR)glob_type_env_cert_appr.json -g $(GLOBS_DIR)glob_comps_cert_appr.json -c 127.0.0.1:5043

am_client_micro:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_micro.json -e $(GLOBS_DIR)glob_type_env_micro.json -g $(GLOBS_DIR)glob_comps_micro.json

am_client_micro_session:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_micro.json -a $(SESSIONS_DIR)session_micro.json

am_client_cds_dynamic:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cds_tpm.json 2>/dev/null

am_client_cds_static:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cds_tpm_static.json 2>/dev/null

am_client_cds_bad_key:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cds_badkey.json -a $(SESSIONS_DIR)session_cds_tpm_bad_sig.json 2>/dev/null

am_client_run_theorem_test:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_run_coq_all_appr.json -a $(SESSIONS_DIR)session_run_theorem_test.json

am_client_run_theorem_test_provision:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_run_coq_all_appr_provision.json -a $(SESSIONS_DIR)session_run_theorem_test.json

resolute_client_help:
	cargo run --release --bin rust-resolute-client -- --help

resolute_client_cert_appr:
	cargo run --release --bin rust-resolute-client -- -r $(RODEO_REQUESTS_DIR)req_rodeo_cert_appr.json -e $(RODEO_ENVS_DIR)env_resolute_cert_appr.json

resolute_client_cert_appr_fixed:
	cargo run --release --bin rust-resolute-client -- -r $(RODEO_REQUESTS_DIR)req_rodeo_cert_appr.json -e $(RODEO_ENVS_DIR)env_resolute_cert_appr.json -c 127.0.0.1:5044

resolute_client_micro:
	cargo run --release --bin rust-resolute-client -- -r $(RODEO_REQUESTS_DIR)req_rodeo_micro.json -e $(RODEO_ENVS_DIR)env_resolute_micro.json

resolute_client_micro_fixed:
	cargo run --release --bin rust-resolute-client -- -r $(RODEO_REQUESTS_DIR)req_rodeo_micro.json -e $(RODEO_ENVS_DIR)env_resolute_micro.json -c 127.0.0.1:5045

resolute_client_run_theorem_test:
	cargo run --release --bin rust-resolute-client -- -r $(RODEO_REQUESTS_DIR)req_rodeo_run_theorem_test.json -e $(RODEO_ENVS_DIR)env_resolute_run_theorem_test.json

clean:
	rm -rf $(BIN)
	cargo clean
