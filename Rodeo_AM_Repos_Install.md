# Steps for installing AM (Attestation Manager) repository dependencies for Rust Rodeo Client

## Github repository dependencies

* cvm (copland virtual machine):  https://github.com/ku-sldg/cvm 
* rust-am-clients:  https://github.com/ku-sldg/rust-am-clients
* asp-libs (attestation service provider libs):  https://github.com/ku-sldg/asp-libs 

## Install Steps

NOTE:  For the smoothest install/usage, clone the above repos to a common parent directory and set an environment variable pointing to that directory called `$AM_REPOS_ROOT` (i.e.): 
```sh
export AM_REPOS_ROOT=<path-to-common-dir>
```


Install Steps:
1) `git clone` each of the above repo dependencies
1) `git checkout` the appropriate branch within each cloned repo (i.e. `git checkout -b <branch_name>`)
1) Follow the installation steps in the top-level README of each cloned repo (this should _hopefully_ just be a `make` command, possibly after some (manual, but minor) installation of tool-specific dependencies).

After those installs, you should have a directory structre like the following:

```sh
$AM_REPOS_ROOT/

    -- cvm/
    -- asp-libs/
    -- rust-am-clients/
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
COMING SOON:  Configuration steps specific to the HAMR/Microkit example.