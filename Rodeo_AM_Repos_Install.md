# Steps for installing AM (Attestation Manager) repository dependencies for Rust Rodeo Client

## Github repository dependencies

* cvm (copland virtual machine):  https://github.com/ku-sldg/cvm 
* rust-am-clients:  https://github.com/ku-sldg/rust-am-clients
* asp-libs (attestation service provider libs):  https://github.com/ku-sldg/asp-libs 
* copland-evidence-tools:  https://github.com/ku-sldg/copland-evidence-tools 

## Install Steps

NOTE:  For the smoothest install/usage, clone the above repos to a common parent directory and set an environment variable pointing to that directory called `$AM_REPOS_ROOT` (i.e.): 
```sh
export AM_REPOS_ROOT=<path-to-common-dir>
```


Install Steps:
1) `git clone` each of the above repo dependencies
1) `git checkout` the appropriate branch within each cloned repo (i.e. `git checkout -b <branch_name>`)
1) Install each repo manually according to its top-level README (rather than the opam install, which may be lagging the latest changes). 

After those installs, you should have a directory structre like the following:

```sh
$AM_REPOS_ROOT/

    -- cvm/
    -- asp-libs/
    -- rust-am-clients/
    -- copland-evidence-tools/
```

## Testing the Installs

To quickly test the above installs, navigate to the `rust-am-clients` repo and run the make target `rodeo_client_test`:

```sh
cd $AM_REPOS_ROOT/rust-am-clients &&
make rodeo_client_test
```

This should build and run a (dummy) attestation scenario that leverages all of the repositories above.  Successful output should look something like:

```
---------------------------------------------------------------
Appraisal Summary: PASSED

magic_appr:
	sys_targ: PASSED
---------------------------------------------------------------
```

## Configure and run an attestation scenario (HAMR/Microkit consistency checker) that requires custom JSON arguments

The goal here is to invoke the rust-rodeo-client executable (part of the rust-am-clients repo) with command line arguments tailored to the HAMR/Microkit attestation scenario.  The rodeo client expects 3 options, as seen by running --help:

```sh
cd $AM_REPOS_ROOT/rust-am-clients &&
make rodeo_client_help
```

* --cvm-filepath - Points to the cvm (Copland Virtual Machine) executable.  This defaults to the manually-installed cvm instance installed at `$AM_REPOS_ROOT/cvm/...` if left unspecified.
* --req-filepath - Specifies a path to a JSON file containing a RodeoClientRequest structure.  The json schema for this structure is in `$AM_REPOS_ROOT/rust-am-clients/json_schemas/rodeo_client_request_schema.json`.  Example/template requests are in `$AM_REPOS_ROOT/rust-am-clients/rodeo_configs/rodeo_requests/`.
* --env-filepath - Specifies a path to a JSON file containing a RodeoEnvironment structure.  These files contain lower-level details to configure attestation components, and should likely not need manual editing for typical Rodeo Client usage.

Example `make` targets using this interface appear in `$AM_REPOS_ROOT/rust-am-clients/Makefile`.  In order to specialize Rodeo artifacts for the HAMR/Microkit scenario, you'll need to edit the RodeoClientRequest structure to pass platform-specific JSON arguments to the attestation tools.  In particular, follow this general workflow:

1) Copy an (abstract) request template to a (concrete) destination directory:
    ```sh
    mkdir $AM_REPOS_ROOT/rust-am-clients/rodeo_configs/rodeo_requests/concrete_requests &&
    cp $AM_REPOS_ROOT/rust-am-clients/rodeo_configs/rodeo_requests/abstract_requests/req_rodeo_micro_abstract.json $AM_REPOS_ROOT/rust-am-clients/rodeo_configs/rodeo_requests/concrete_requests/req_rodeo_micro_concrete.json
    ```
1) Edit the relevant fields to fill in platform-specific paths.  For the HAMR/Microkit scenario, the important fileds are: `paths` and `filepath_golden`.  The `hashdir` ASP (Attestation Service Provider) has two "targets" in this scenario, namely `aadl_dir_targ` and `microkit_dir_targ`.  The `paths` field tells the attestation tools where to find the target directories to measure, and `filepath_golden` specifies where to look for golden hash values during evidence appraisal.
1) Invoke the rust-rodeo-client pointing to this (new) concrete request.  See the `make` target `rodeo_client_hamr` for example usage.

In order to run the HAMR/Microkit appraisal scenario, you'll first need to _provision_ the golden evidence values.  This amounts to running a provisioning rodeo scenario following the workflow steps above.  The relevant RodeoEnvironment and (abstract) RodeoClientRequest structures for provisioning are in `rodeo_configs/rodeo_envs/env_rodeo_micro_provision.json` and `rodeo_configs/rodeo_requests/abstract_requests/req_rodeo_micro_provision_abstract`.  To concretize the provisioning RodeoEnvironment structure, you'll need to fill in the `paths` field for both of the `hashdir` ASP targets, and the `filepath_golden` field for both of the `provision` ASP targets.  After running the provisioning protocol, the files at the paths specified by `filepath_golden` should be populated with golden hash values.  You can safely ignore any "AppraisalSummary" output as it is irrelevant for provisioning.

Finally, run the appraisal protocol (pointing to the same `filepath_golden` paths).  Upon success, the rodeo client executable will output (to stdout) a RodeoClientResponse JSON structure (see `$AM_REPOS_ROOT/rust-am-clients/json_schemas/rodeo_client_response_schema.json`), along with some additional logging and a pretty-printed "Appraisal Summary".  To suppress the additional logging and Appraisal Summary (sent via stderr), redirect stderr to /dev/null (i.e. `<command> 2> /dev/null`).  This will output the RodeoClientResponse structure (and only that structure) to stdout.

The important field of the RodeoClientResponse is the boolean `"RodeoClientResponse_success"`, indicating a successful appraisal judgement of all measurement targets.  This structure also contains the more detailed request/response structures sent to the cvm executable, where the request contains the Copland protocol Term and Attestaion Session configured by Rodeo, and the response contains Copland Evidence (and meta-evidence).