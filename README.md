# `libninja`

`libninja` is a tool for generating world-class, idiomatic client libraries from OpenAPI specs.

The best way to see it in action is to see what it produces.

[`plaid-rs`](https://github.com/libninjacom/plaid-rs) is generated entirely by Libninja. This includes:
- The client library itself
- Idiomatic interface, where required arguments are passed directly or as part of a struct, and optional arguments are included via method chaining.
- Documentation is published online (docs.rs), and crate is published to registry (crates.io)
- `examples/` folder containing an example for every API endpoint
- The API client has the ability to record/replay requests, greatly aiding development for end users.
- API documentation is included in function docstrings, so it's available inline in the editor. The docstrings also include links to plaid's hosted API documentation.
- Github Action .yaml files to run tests and publish the package to package registries
- README that includes badges (that showcase a Green passing build) and usage examples

All of that is created with this command:

```bash
libninja gen --lang rust --repo libninjacom/plaid-rs -o . Plaid ~/path/to/plaid/openapi.yaml
```

# Installation

```
cargo install libninja-cli
```

Use the command line help to see required arguments & options when generating libraries.

The open source version builds client libraries for Rust. Libninja also supports other languages with a commercial license. Reach out at the email in author Github profile.
