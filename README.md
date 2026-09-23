# Germ

[![crates.io](https://img.shields.io/crates/v/germ.svg)](https://crates.io/crates/germ)
[![docs.rs](https://docs.rs/germ/badge.svg)](https://docs.rs/germ)
[![github.com](https://github.com/gemrest/germ/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/gemrest/germ/actions/workflows/check.yaml)

The Ultimate Gemini Toolkit

Germ is a toolkit for the Gemini protocol which aims to have a little something
for everyone. At the moment, Germ has **ZERO** dependencies unless you use the
`request` or `blocking` feature, and Germ will continue to try its hardest to
have as few dependencies as possible.

## Features

- AST builder to easily construct and manipulate AST trees from raw Gemtext
- Converters to easily convert from Gemtext to markup formats such as HTML or
  Markdown
- Blocking and non-blocking request suite
- Structured meta section manipulation
- And more!

Check out the rest of the features in the Features section under Usage

## Usage

Current version:
[![crates.io](https://img.shields.io/crates/v/germ.svg)](https://crates.io/crates/germ)

```toml
# Cargo.toml

[dependencies]
# To enable only the base (default) features: ast, convert, meta, request
# germ = "*" # Use current version show above!

# To enable only certain features
[dependencies.germ]
version = "*" # Use current version show above!
default-features = false
features = ["ast"] # Enable the features you would like to use!
```

### Features

| Feature    | Description                                                           |
| ---------- | --------------------------------------------------------------------- |
| `default`  | `ast`, `convert`, `meta`, `request`                                   |
| `ast`      | Construct AST trees from raw Gemtext                                  |
| `blocking` | Blocking equivalent of `request`                                      |
| `convert`  | Convert Gemtext to markup formats such as HTML or Markdown            |
| `request`  | Make Gemini requests, get sane, structured results                    |
| `meta`     | Structure-ise a Gemini response's meta section                        |
| `macros`   | Macros to aid with various Germ-related functionalities               |
| `quick`    | Tiny functions to create valid Gemtext elements from structured input |

Gemini requests verify server certificates against Mozilla CA roots by default.
For a self-signed capsule, add its certificate in DER form to a
`rustls::RootCertStore`, then pass that store to
`request::blocking::request_with_roots` or
`request::non_blocking::request_with_roots`. The store is a trust anchor, not an
exact certificate fingerprint pin. Save and review the certificate separately
before trusting it; the default request functions do not remember certificates
between calls.

HTML conversion escapes Gemtext content. It makes relative links and Gemini,
Gopher, HTTP, HTTPS, mailto, and FTP links clickable; other schemes are shown
as text.

### Examples

Thoroughly commented examples can be found within the
[`examples/`](https://github.com/gemrest/germ/tree/main/examples) directory.

Examples can be run by name using the example just task.
(e.g., `just example ast_to_gemtext`)

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
