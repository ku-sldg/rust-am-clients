BIN := target

ifdef AM_REPOS_ROOT
CVM_EXE_PATH := $(AM_REPOS_ROOT)/cvm/_build/install/default/bin/cvm
GOLDEN_EVIDENCE_DIR := $(AM_REPOS_ROOT)/rust-am-clients/goldenFiles/
OUTPUTS_DIR := $(AM_REPOS_ROOT)/rust-am-clients/testing/outputs/
else
$(error "ERROR:  AM_REPOS_ROOT environment variable not set!")
endif

# Variables used in make targets to configure various clients
RODEO_CONFIGS_DIR=$(AM_REPOS_ROOT)/rust-am-clients/rodeo_configs/
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
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_micro.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/micro_args_concrete.json -a 2> /dev/null

rodeo_client_hamr_verbose:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_micro.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/micro_args_concrete.json -a -o $(OUTPUTS_DIR)

rodeo_client_hamr_auto_provision:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_micro.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/micro_args_concrete.json -p $(GOLDEN_EVIDENCE_DIR)micro_evidence_golden.json

rodeo_client_theorem_auto_provision:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_theorem.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/theorem_args_concrete.json  -p $(GOLDEN_EVIDENCE_DIR)theorem_evidence_golden.json

rodeo_client_theorem_verbose:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_theorem.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/theorem_args_concrete.json -a

rodeo_client_verus:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_verus.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/verus_args_concrete.json -a

rodeo_client_verus_auto_provision:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_verus.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/verus_args_concrete.json -p $(GOLDEN_EVIDENCE_DIR)verus_evidence_golden.json

rodeo_client_autoverus:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_autoverus.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/autoverus_args_concrete.json

clean:
	rm -rf $(BIN)
	cargo clean
