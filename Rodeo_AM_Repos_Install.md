# Steps for setting up AM Repository dependnecies for Rust Rodeo Client

## Github repository dependencies

* cvm (copland virtual machine):  https://github.com/ku-sldg/cvm 
* rust-am-clients:  https://github.com/ku-sldg/rust-am-clients
* asp-libs (attestation service provider libs):  https://github.com/ku-sldg/asp-libs 

## Install Steps

Note:  for the smoothest install/usage, clone the above repos to a common parent directory and set an environment variable pointing to that directory called `$AM_REPOS_ROOT` (i.e.): 
```sh
export AM_REPOS_ROOT=<path-to-common-dir>
```


Install Steps:
1) `git clone` each of the above repo dependencies
1) `git checkout` the appropriate branch within each cloned repo (i.e. `git checkout -b <branch_name>`)
1) Follow the installation steps in the top-level README of each cloned repo (this should _hopefully_ just be a `make` command, possibly with some installation of dependencies).