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
