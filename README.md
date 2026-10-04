# Germ

[![crates.io](https://img.shields.io/crates/v/germ.svg)](https://crates.io/crates/germ)
[![docs.rs](https://docs.rs/germ/badge.svg)](https://docs.rs/germ)
[![github.com](https://github.com/gemrest/germ/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/gemrest/germ/actions/workflows/check.yaml)

Germ parses and generates Gemtext, converts it to HTML or Markdown, handles
response metadata, and makes Gemini requests.

## Usage

```toml
[dependencies]
germ = "0.5.1"

# For Gemtext parsing only, use this instead.
# germ = { version = "0.5.1", default-features = false, features = ["ast"] }
```

### Features

| Feature    | Description                         |
| ---------- | ----------------------------------- |
| `default`  | `ast`, `convert`, `meta`, `request` |
| `ast`      | Parse and generate Gemtext          |
| `blocking` | Blocking Gemini requests            |
| `convert`  | HTML and Markdown rendering         |
| `request`  | Asynchronous Gemini requests        |
| `meta`     | Parse and format response metadata  |
| `macros`   | Gemtext and conversion macros       |
| `quick`    | Build individual Gemtext lines      |

`convert` enables `ast`; `macros` enables `ast` and `convert`.

Requests use CA verification by default. For trust on first use, pass a
`CertificateStore` to `request_with_tofu`.

### Examples

Run any [example](examples/) with `just example <name>`.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
