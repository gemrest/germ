# Germ

[![crates.io](https://img.shields.io/crates/v/germ.svg)](https://crates.io/crates/germ)
[![docs.rs](https://docs.rs/germ/badge.svg)](https://docs.rs/germ)
[![github.com](https://github.com/gemrest/germ/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/gemrest/germ/actions/workflows/check.yaml)

Germ parses and generates Gemtext, converts it to HTML or Markdown, handles
response metadata, and makes Gemini requests. Its default features include
the request client and its network and TLS dependencies. Disable default
features to use `ast`, `convert`, `meta`, or `quick` without normal dependencies.

## Usage

```toml
[dependencies]
germ = "0.4.8"
```

To select features without the default request client:

```toml
[dependencies]
germ = { version = "0.4.8", default-features = false, features = ["ast"] }
```

### Features

| Feature    | Description                                                           |
| ---------- | --------------------------------------------------------------------- |
| `default`  | `ast`, `convert`, `meta`, `request`                                   |
| `ast`      | Construct AST trees from raw Gemtext                                  |
| `blocking` | Make blocking Gemini requests                                          |
| `convert`  | Convert Gemtext to HTML or Markdown                                    |
| `request`  | Make asynchronous Gemini requests                                      |
| `meta`     | Parse and format Gemini response metadata                              |
| `macros`   | Generate AST and converted output with macros                          |
| `quick`    | Build individual Gemtext lines                                         |

Gemini requests verify server certificates against Mozilla CA roots by default.
For a self-signed capsule, add its certificate in DER form to a
`rustls::RootCertStore`, then pass that store to
`request::blocking::request_with_roots` or
`request::non_blocking::request_with_roots`. The store is a trust anchor, not an
exact certificate fingerprint pin. Save and review the certificate separately
before trusting it; the default request functions do not remember certificates
between calls.

Requests time out after 30 seconds and have a 16 MiB response limit, including
the header. To change these limits or the trusted roots, set the fields of
`request::RequestOptions` and pass it to
`request::blocking::request_with_options` or
`request::non_blocking::request_with_options`. The blocking system DNS lookup
may outlast the timeout before it returns.

For trust on first use, implement `request::CertificateStore` with durable
storage keyed by hostname and port, then pass it to
`blocking::request_with_tofu` or `non_blocking::request_with_tofu` alongside
`RequestOptions`. These methods verify the presented certificate and complete
the TLS handshake before saving a first-use certificate or checking an existing
one, and before sending the URL. They reject an unexpected certificate change
until the stored certificate expires. An expired certificate is replaced after
the new one passes validation. The store must persist each update before
returning; callers can remove a pin deliberately when changing trust early.
Store methods run synchronously, including in the async request path.
TOFU uses the timeout and response size limits from `RequestOptions` but
ignores its `root_certificates` field.

HTML conversion escapes Gemtext content. HTML and Markdown conversion emit
links for relative targets and Gemini, Gopher, HTTP, HTTPS, mailto, and FTP
URLs; other schemes are shown as text. Markdown conversion also escapes link
labels and destinations.

`Meta` parses quoted MIME parameters and quotes values as needed when formatting
them.

`quick::link(url, label)` takes the URL first and an optional label second.

Unknown Gemini status codes retain their numeric value. Use
`Status::category()` to inspect the response category indicated by the first
digit.

### Examples

Runnable examples can be found within the
[`examples/`](https://github.com/gemrest/germ/tree/main/examples) directory.

Run one with `just example ast_to_gemtext`.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
