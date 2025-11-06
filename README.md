# rust-am-clients
A repository for client implementations written in Rust that interact with Attestation Manager (AM) and Attestation Service Provider (ASP) servers.


## Building

Once you have Rust installed (https://www.rust-lang.org/tools/install), simply typing `make` should build all the executable targets specified in this workspace.

Next, follow the installation instructions in the [cvm](https://github.com/ku-sldg/cvm) and [asp-libs](https://github.com/ku-sldg/asp-libs) repositories as specified in their READMEs.

Finally, set the `AM_REPOS_ROOT` environment variable as the parent directory to these repos as depicted here:

```
AM_REPOS_ROOT/
├── cvm/
├── rust-am-clients/
├── asp-libs/
```

## Testing

Try running the `rodeo_client_test` make target as follows:

```bash
make rodeo_client_test
```

Successful output should be some printed JSON logging followed by something like: 

``Protocol completed successfully!``
