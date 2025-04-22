# rust-am-clients
A repository for client implementations written in Rust that interact with Attestation Manager (AM) and Attestation Service Provider (ASP) servers.

Corresponding AM servers are implemented [here](https://github.com/ku-sldg/am-cakeml) and extracted from formal specs [here](https://github.com/ku-sldg/copland-avm).

Corresponding ASP servers are implemented [here](https://github.com/ku-sldg/asp-libs).


## Building

Once you have Rust installed (https://www.rust-lang.org/tools/install), simply typing `make` should build all the executable targets specified in this workspace.

Next, build the [am-cakeml](https://github.com/ku-sldg/am-cakeml) and [asp-libs](https://github.com/ku-sldg/asp-libs) repo dependencies as specified in their READMEs.

## Testing

First, set the environment variable `AM_CLIENTS_ROOT` to point to the top-level of this repo (`rust-am-clients/`).

Once am-cakeml and asp-libs are installed and built, start a collection of AM servers via am-cakeml (i.e. from `am-cakeml/tests/` run:  `Demo.sh -t <target>`).  This will start a tmux session with an appropriate number of tmux windows for each AM involved in the `<target>` protocol specified.  Then in a separate terminal initiate a corresponding client session (i.e. `make am_client_<target>`).  

Currently, if using the development branch `copland-lib-pub-adts` for all repo dependencies (am-cakeml, asp-libs), moderate testing has been performed for `<target>:= cert, cert_appr, micro`.  See the rust-am-clients [Makefile](https://github.com/ku-sldg/rust-am-clients/blob/main/Makefile) for example Make targets (i.e. using the resolute client on the above targets:  i.e. `make resolute_client_cert_appr`).  Successful output for am_client sessions should be something like:  `ProtocolRunResponse { ... }`.  For resolute_client sessions, something like:  `ResoluteClientResponse Success: true/false` indicates completion of the attestation scneario and appraisal judgement.