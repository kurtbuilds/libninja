# `libninja`

`libninja` is a tool for generating world-class, idiomatic client libraries from OpenAPI specs.

The best way to see it in action is to see what it produces.

[`plaid-rs`](https://github.com/libninjacom/plaid-rs) is generated entirely by Libninja. This includes:

- The client library itself
- Idiomatic interface, where required arguments are passed directly or as part of a struct, and optional arguments are
  included via method chaining.
- Documentation is published online (docs.rs), and crate is published to registry (crates.io)
- `examples/` folder containing an example for every API endpoint
- The API client has the ability to record/replay requests, greatly aiding development for end users.
- API documentation is included in function docstrings, so it's available inline in the editor. The docstrings also
  include links to plaid's hosted API documentation.

All of that is created with this command:

```bash
libninja gen Plaid ~/path/to/plaid/openapi.yaml
```

# Installation

```
cargo install --git https://github.com/kurtbuilds/libninja
```

Use the command line help to see required arguments & options when generating libraries.

# Usage

## Deriving traits for generated structs

You can derive traits for the generated structs by passing them using one (or many) `--derive` arguments:

```bash
libninja gen --derive oasgen::OaSchema --derive faker::Dummy Plaid ~/path/to/plaid/openapi.yaml 
```

Make sure to add the referenced crate(s) (and any necessary features) to your `Cargo.toml`:

```bash
cargo add oasgen --features chrono
```

Then, the traits will be added to the `derive` attribute on the generated `model` and `request` structs:

```rust
use serde::{Serialize, Deserialize};
use super::Glossary;
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct ListGlossariesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glossaries: Option<Vec<Glossary>>,
}
```

## Customizing Files

During codegen, `libninja` will examine the target directory for files or content it should keep (effectively, using the
existing crate as a template). It looks for two directives, `libninja: static` and `libninja: after`.

If `libninja` encounters `libninja: static`, it will skip generation entirely, and keep the existing file as-is.

If `libninja` encounters `libninja: after`, it will overwrite any code encountered after that directive, replacing
it with the generated code. Generally, use this when you want to customize the imports or add additional structs or
functions to the file.

Importantly, libninja removes outdated code, so any handwritten file is not marked with `libninja: static` will be
removed.

### Customize the OpenAPI spec

Most OpenAPI specs you encounter in the real world are not perfect, and sometimes are entirely broken. You can manually
modify the script or write a script to do so.

The preferred way is to write a script to modify the spec. You can use `serde` to deserialize the spec, modify it, and
then serialize it back to disk. This way, you can rerun the same modifications every time the spec changes. This script
can be a standalone crate (which can live in the same repo), or part of build.rs.

If the spec is invalid and doesn't deserialize, deserialize it as a `serde_json::Value` to make it compliant, and then
deserialize again (serde_json::from_value) into a OpenAPI spec for further processing.

Alternatively, manually modifying the script is great for one-off changes, but your target spec might be evolving over
time. You can use `git` diffings to partially address this, but it's not ideal.
If you go this route, the [openapi](https://github.com/kurtbuilds/openapiv3_cli) cli tool can help.