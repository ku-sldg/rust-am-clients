BIN := target

ifdef AM_REPOS_ROOT
CVM_EXE_PATH := $(AM_REPOS_ROOT)/cvm/_build/install/default/bin/cvm
GOLDEN_EVIDENCE_DIR := $(AM_REPOS_ROOT)/rust-am-clients/goldenFiles/
else
$(error "ERROR:  AM_REPOS_ROOT environment variable not set!")
endif

# Variables used in make targets to configure various clients
RODEO_REQUESTS_DIR=$(AM_REPOS_ROOT)/rust-am-clients/rodeo_configs/rodeo_requests/
RODEO_ENVS_DIR=$(AM_REPOS_ROOT)/rust-am-clients/rodeo_configs/rodeo_envs/

default:
	cargo build --release --workspace

all: 
	cargo build --release

debug: 
	cargo build 

test:
	make default
	cargo test --workspace

rodeo_client_help:
	cargo run --release --bin rust-rodeo-client -- --help

rodeo_client_test:
	cargo run --release --bin rust-rodeo-client -- -r $(RODEO_REQUESTS_DIR)/abstract_requests/req_rodeo_attest_abstract.json -e $(RODEO_ENVS_DIR)env_rodeo_attest.json

rodeo_client_hamr:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_micro_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_micro.json 2> /dev/null

rodeo_client_hamr_verbose:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_micro_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_micro.json

rodeo_client_hamr_auto_provision:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_micro_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_micro_auto_provision.json -p $(GOLDEN_EVIDENCE_DIR)micro_evidence_golden.json

rodeo_client_theorem_verbose:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_theorem_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_theorem.json

rodeo_client_theorem_provision_verbose:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_theorem_provision_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_theorem_provision.json

rodeo_client_hamr_provision_verbose:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_micro_provision_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_micro_provision.json

rodeo_client_verus_auto_provision:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_verus_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_verus.json -p $(GOLDEN_EVIDENCE_DIR)verus_evidence_golden.json

rodeo_client_verus_verbose:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_verus_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_verus.json -a

clean:
	rm -rf $(BIN)
	cargo clean
