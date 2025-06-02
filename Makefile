BIN := target

# Variables used in make targets to configure various clients
PROTOCOLS_DIR=testing/protocols/noargs/
GLOBS_DIR=testing/globals/
SESSIONS_DIR=testing/attestation_sessions/
ASP_ARGS_DIR=testing/asp_args/concretized_args/
ASP_ARGS_DUMMY_DIR=testing/asp_args/
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

am_client_attest:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_attest_noargs.json -a $(SESSIONS_DIR)session_cert_appr.json -s 127.0.0.1:5000

am_client_attest_remote:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_attest_remote_noargs.json -a $(SESSIONS_DIR)session_attest_remote.json -s 127.0.0.1:5000

am_client_attest_remote_multinode:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_attest_remote_multinode_noargs.json -a $(SESSIONS_DIR)session_attest_remote_multinode.json -s 127.0.0.1:5000

am_client_cert:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert_noargs.json -a $(SESSIONS_DIR)session_cert_appr.json -s 127.0.0.1:5000

am_client_cert_fixed:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert_noargs.json -c 127.0.0.1:5042

am_client_cert_appr:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert_appr_noargs.json -a $(SESSIONS_DIR)session_cert_appr.json -s 127.0.0.1:5000

am_client_cert_appr_delegated:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert_noargs.json -a $(SESSIONS_DIR)session_cert_appr.json -s 127.0.0.1:5000 -r 127.0.0.1:5000 -d $(ASP_ARGS_DUMMY_DIR)cert_appr_args.json -m

am_client_cert_appr_appsumm:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert_appr_noargs.json -a $(SESSIONS_DIR)session_cert_appr.json -s 127.0.0.1:5000 -m

am_client_cert_appr_fixed:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cert_appr.json -e $(GLOBS_DIR)glob_type_env_cert_appr.json -g $(GLOBS_DIR)glob_comps_cert_appr.json -c 127.0.0.1:5043

am_client_micro:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_micro_noappr_noargs.json -a $(SESSIONS_DIR)session_micro.json -r 127.0.0.1:5000 -b $(ASP_ARGS_DIR)micro_args.json -d $(ASP_ARGS_DIR)micro_args_appr.json

am_client_micro_glob_types:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_micro_noappr_noargs.json -e $(GLOBS_DIR)glob_type_env_micro.json -g $(GLOBS_DIR)glob_comps_micro.json -r 127.0.0.1:5000 -b $(ASP_ARGS_DIR)micro_args.json -d $(ASP_ARGS_DIR)micro_args_appr.json

am_client_cds_dynamic:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cds_tpm.json 2>/dev/null

am_client_cds_static:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cds_tpm_static.json 2>/dev/null

am_client_cds_bad_key:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_cds_badkey.json -a $(SESSIONS_DIR)session_cds_tpm_bad_sig.json 2>/dev/null

am_client_run_theorem_test:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_run_coq_all_noargs.json -a $(SESSIONS_DIR)session_run_theorem_test.json -r 127.0.0.1:5000 -b $(ASP_ARGS_DIR)run_theorem_test_args_concretized.json -d $(ASP_ARGS_DIR)run_theorem_test_args_appr_concretized.json

am_client_run_theorem_test_provision:
	cargo run --release --bin rust-am-client -- -t $(PROTOCOLS_DIR)protocol_theorem_provision_evidence_noargs.json -a $(SESSIONS_DIR)session_run_theorem_test_provision.json -b $(ASP_ARGS_DIR)run_theorem_test_provision_args_concretized.json 

rodeo_client_help:
	cargo run --release --bin rust-rodeo-client -- --help

rodeo_client_cert_appr:
	cargo run --release --bin rust-rodeo-client -- -q $(RODEO_REQUESTS_DIR)req_rodeo_cert_appr.json -e $(RODEO_ENVS_DIR)env_rodeo_cert_appr.json

rodeo_client_cert_appr_fixed:
	cargo run --release --bin rust-rodeo-client -- -q $(RODEO_REQUESTS_DIR)req_rodeo_cert_appr.json -e $(RODEO_ENVS_DIR)env_rodeo_cert_appr.json -c 127.0.0.1:5044

rodeo_client_micro:
	cargo run --release --bin rust-rodeo-client -- -q $(RODEO_REQUESTS_DIR)req_rodeo_micro.json -e $(RODEO_ENVS_DIR)env_rodeo_micro.json -r 127.0.0.1:5000

rodeo_client_micro_fixed:
	cargo run --release --bin rust-rodeo-client -- -q $(RODEO_REQUESTS_DIR)req_rodeo_micro.json -e $(RODEO_ENVS_DIR)env_rodeo_micro.json -c 127.0.0.1:5045 -r 127.0.0.1:5000

rodeo_client_run_theorem_test:
	cargo run --release --bin rust-rodeo-client -- -q $(RODEO_REQUESTS_DIR)req_rodeo_run_theorem_test.json -e $(RODEO_ENVS_DIR)env_rodeo_run_theorem_test.json -r 127.0.0.1:5000

clean:
	rm -rf $(BIN)
	cargo clean
