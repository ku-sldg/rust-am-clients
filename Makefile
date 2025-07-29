BIN := target

ifdef AM_REPOS_ROOT
CVM_EXE_PATH := $(AM_REPOS_ROOT)/cvm/_build/install/default/bin/cvm
else
$(error "ERROR:  AM_REPOS_ROOT environment variable not set!")
endif

# Variables used in make targets to configure various clients
#CVM_DIR=/Users/adampetz/Documents/Summer_2025/cvm/_build/install/default/bin/cvm
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

rodeo_client_hamr:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_micro_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_micro.json 2> /dev/null

rodeo_client_hamr_verbose:
	cargo run --release --bin rust-rodeo-client -- -c $(CVM_EXE_PATH) -r $(RODEO_REQUESTS_DIR)concrete_requests/req_rodeo_micro_concrete.json -e $(RODEO_ENVS_DIR)env_rodeo_micro.json
clean:
	rm -rf $(BIN)
	cargo clean
