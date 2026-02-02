BIN := target

ifdef ASP_BIN
ASP_BIN_PATH := $(ASP_BIN)
else
$(error "ERROR:  ASP_BIN environment variable not set!")
endif

CURRENT_DIR := $(CURDIR)

# Variables used in make targets to configure various clients
MANIFEST_PATH := $(CURRENT_DIR)/testing/manifests/Manifest_P0.json
GOLDEN_EVIDENCE_DIR := $(CURRENT_DIR)/goldenFiles/
OUTPUTS_DIR := $(CURRENT_DIR)/testing/outputs/
RODEO_CONFIGS_DIR=$(CURRENT_DIR)/rodeo_configs/
RODEO_REQUESTS_DIR=$(CURRENT_DIR)/rodeo_configs/rodeo_requests/
RODEO_ENVS_DIR=$(CURRENT_DIR)/rodeo_configs/rodeo_envs/

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
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -r $(RODEO_REQUESTS_DIR)/abstract_requests/req_rodeo_attest_abstract.json -e $(RODEO_ENVS_DIR)env_rodeo_attest.json

rodeo_client_test_granular:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_test.json -s $(RODEO_CONFIGS_DIR)sessions/session_test.json -o $(OUTPUTS_DIR)

rodeo_client_hamr:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_micro.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/micro_args_concrete.json -a 2> /dev/null

rodeo_client_hamr_verbose:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_micro.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/micro_args_concrete.json -a -o $(OUTPUTS_DIR)

rodeo_client_hamr_auto_provision:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_micro.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/micro_args_concrete.json -p $(GOLDEN_EVIDENCE_DIR)micro_evidence_golden.json

rodeo_client_theorem_auto_provision:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_theorem.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/theorem_args_concrete.json  -p $(GOLDEN_EVIDENCE_DIR)theorem_evidence_golden.json

rodeo_client_theorem_verbose:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_theorem.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/theorem_args_concrete.json -a

rodeo_client_verus:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_verus.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/verus_args_concrete.json -a

rodeo_client_verus_workflow:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_verus_workflow.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/verus_workflow_args_concrete.json -o $(OUTPUTS_DIR) -a

rodeo_client_verus_workflow_auto_provision:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_verus_workflow.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/verus_workflow_args_concrete.json -p $(GOLDEN_EVIDENCE_DIR)verus_workflow_evidence_golden.json

rodeo_client_verus_compare:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_verus_compare.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/verus_compare_args_concrete.json -a

rodeo_client_verus_auto_provision:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_verus.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/verus_args_concrete.json -p $(GOLDEN_EVIDENCE_DIR)verus_evidence_golden.json

rodeo_client_autoverus:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_autoverus.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/autoverus_args_concrete.json

rodeo_client_readfile_range:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_readfile_range.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/readfile_range_args_concrete.json -o $(OUTPUTS_DIR) -a

rodeo_client_readfile_range_provision:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_readfile_range.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/empty_args.json -o $(OUTPUTS_DIR) -p $(GOLDEN_EVIDENCE_DIR)readfile_range_evidence_golden.json

rodeo_client_hamr_readfile_range:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_hamr_readfile_range_test.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/empty_args.json -o $(OUTPUTS_DIR) -a

rodeo_client_hamr_readfile_range_provision:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_hamr_readfile_range_test.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/empty_args.json -o $(OUTPUTS_DIR) -p $(GOLDEN_EVIDENCE_DIR)hamr_readfile_range_evidence_golden.json

rodeo_client_hamr_readfile_range_short:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_hamr_readfile_range_test_short.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/empty_args.json -o $(OUTPUTS_DIR) -a

rodeo_client_hamr_readfile_range_short_provision:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_hamr_readfile_range_test_short.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/empty_args.json -o $(OUTPUTS_DIR) -p $(GOLDEN_EVIDENCE_DIR)hamr_readfile_range_evidence_golden.json 

rodeo_client_hamr_readfile_range_medium:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_hamr_readfile_range_test_medium.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/empty_args.json -o $(OUTPUTS_DIR) -a

rodeo_client_hamr_readfile_range_medium_provision:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_hamr_readfile_range_test_medium.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/empty_args.json -o $(OUTPUTS_DIR) -p $(GOLDEN_EVIDENCE_DIR)hamr_readfile_range_evidence_golden.json

rodeo_client_hamr_contracts:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_hamr_contracts.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/empty_args.json -o $(OUTPUTS_DIR) -a

rodeo_client_hamr_contracts_provision:
	cargo run --release --bin rust-rodeo-client -- -l $(ASP_BIN_PATH) -m $(MANIFEST_PATH) -t $(RODEO_CONFIGS_DIR)protocols/protocol_hamr_contracts.json -s $(RODEO_CONFIGS_DIR)sessions/session_union.json -g $(RODEO_CONFIGS_DIR)asp_args/concrete/empty_args.json -o $(OUTPUTS_DIR) -p $(GOLDEN_EVIDENCE_DIR)hamr_contracts_golden_evidence.json


clean:
	rm -rf $(BIN)
	cargo clean
