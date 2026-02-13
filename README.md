# rust-am-clients
A repository for client implementations written in Rust that interact with Attestation Manager (AM) and Attestation Service Provider (ASP) servers.


## Building

Once you have Rust installed (https://www.rust-lang.org/tools/install), simply typing `make` should build all the executable targets specified in this workspace.

To install necessary runtime dependencies for these executables, follow the installation instructions in the [cvm](https://github.com/ku-sldg/cvm) and [asp-libs](https://github.com/ku-sldg/asp-libs) repositories as specified in their READMEs.

## Testing

Before testing a rust client executable, you'll need to make sure its `cvm` and `asp-libs` dependencies are visible (NOTE:  An alternative is to pass paths to these dependencies explicitly as CLI arguments to a client executable).

First, make sure the `cvm` executable is on your PATH:

```bash
which cvm
```

Next, set the `ASP_BIN` environment variable to point to your ASP executables installed under `asp-libs`:

```bash
export ASP_BIN=<path_to_asp-libs>/target/release/
```


Now try running the `rodeo_client_test` make target as follows:

```bash
make rodeo_client_test
```

Successful output should be some JSON logging followed by something like: 

``Protocol completed successfully!``

## Steps for testing the RODEO-HAMR workflow

1. Install and test the `rust-rodeo-client` executable and its dependencies (see above)
1. Clone the [INSPECTA-models](https://github.com/loonwerks/INSPECTA-models) repository, and locate the `attestation/` directory for the codegen project you wish to attest (i.e. for the isolette project this would be:  `INSPECTA-models/isolette/hamr/microkit/attestation`).  Below we assume this path is in an environment variable called `$HAMR_ATTESTATION_ROOT`.
1. Identify the filename of the HAMR attestation report at `$HAMR_ATTESTATION_ROOT` (i.e. `sysml_attestation_report.json`).
1. From the top-level directory of the `rust-am-clients` repository, run HAMR contract provisioning:

    ```bash
    cargo run --release --bin rust-rodeo-client -- --hamr-report-filepath $HAMR_ATTESTATION_ROOT/sysml_attestation_report.json -p $HAMR_ATTESTATION_ROOT/hamr_maestro_golden_evidence.json
    ```

    Note:  the `--hamr-root` CLI arg expects the hamr attestation root directory path, `--hamr-model-filename` expects the NAME of the attestation report at `$HAMR_ATTESTATION_ROOT`, and the `-p` option (provision) expects a path for the output golden evidence file.
    After running the above command, MAESTRO provisioning will populate two output files in the `$HAMR_ATTESTATION_ROOT` directory, namely `hamr_maestro_term.json` (the attestation protocol encoding -- filename is hardcoded for now) and `hamr_maestro_golden_evidence.json` (the golden evidence structure -- filename passed to provisioning).
1. Again in `rust-am-clients/`, run HAMR contract appraisal:

    ```bash
    cargo run --release --bin rust-rodeo-client -- -t $HAMR_ATTESTATION_ROOT/hamr_maestro_term.json -a
    ```
    The `-t` option points to the Copland protocol term generated in the previous step.  `-a` tells the MAESTRO tools to perform evidence appraisal against the golden evidence file generated during provisioning above (the path to this golden evidence file is embedded into the protocol term during provisioning).  
1. Check the `rust-am-clients/testing/outputs/` directory for the newly-generated file called `appsumm_response.json` (the `-o` option can override this default output directory).  This is an AppraisalSummary Response JSON structure.  The crucial field of this JSON object is `"APPRAISAL_RESULT"` which captures the overall appraisal judgement for the HAMR contract file slices as a boolean.  The JSON schema for the AppraisalSummary Response can be found [here](https://github.com/ku-sldg/rust-am-clients/blob/main/json_schemas/appsumm_response_schema.json).

