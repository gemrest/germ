import? 'cargo.just'

set allow-duplicate-recipes := true

default:
  @just --list

fetch:
  curl https://raw.githubusercontent.com/Fuwn/justfiles/a6ca8a1b0475966ad10b68c44311ba3cb8b72a31/cargo.just > cargo.just

fmt:
  cargo +nightly fmt

test:
  cargo test --all-features

docs:
  cargo +nightly doc --open --no-deps

example example:
  cargo run --example {{ example }} --all-features

diff url="gemini://fuwn.me/":
  cargo run --example request_blocking_to_gemtext_from_ast --all-features -- {{ url }}
