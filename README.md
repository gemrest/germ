# Germ

[![crates.io](https://img.shields.io/crates/v/germ.svg)](https://crates.io/crates/germ)
[![docs.rs](https://docs.rs/germ/badge.svg)](https://docs.rs/germ)
[![github.com](https://github.com/gemrest/germ/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/gemrest/germ/actions/workflows/check.yaml)

The Ultimate Gemini Toolkit.

Germ is a toolkit for the Gemini protocol which aims to have a little something
for everyone. At the moment, Germ has **ZERO** dependencies (unless you use the
`request` feature), and Germ will continue to try its hardest to have as few
dependencies as possible.

## Features

- AST builder to easily construct AST trees from raw Gemtext.
- Converters to easily convert from Gemtext to markup formats such as HTML or
  Markdown.
- More to come!

## Usage

Current version:
[![crates.io](https://img.shields.io/crates/v/germ.svg)](https://crates.io/crates/germ)

```toml
# Cargo.toml

[dependencies]
# To enable all features
# germ = "*" # Use current version show above!

# To enable certain features
[dependencies.germ]
version = "*" # Use current version show above!
default-features = false
features = ["ast"] # Enable the features you would like to use!
```

### Features

| Feature   | Description                                                      |
| --------- | ---------------------------------------------------------------- |
| `ast`     | Construct AST trees from raw Gemtext.                            |
| `convert` | Convert from Gemtext to markup formats such as HTML or Markdown. |
| `request` | Make Gemini requests and get sane, structured results.           |
| `meta`    | Structure-ize a Gemini response's meta section                   |
| `macros`  | Macros to aid with various Germ-related functionalities          |
| `quick`   | Quick functions to create valid Gemtext elements from input      |
| `sync`    | An asynchronous version of `request`                             |

### Examples

Examples can be found within the
[`examples/`](https://github.com/gemrest/germ/tree/main/crates/germ/examples) directory.

## License

This project is licensed with the
[GNU General Public License v3.0](https://github.com/gemrest/germ/blob/main/LICENSE).
