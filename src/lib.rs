#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code,
  clippy::all,
  clippy::nursery,
  clippy::pedantic
)]
#![recursion_limit = "128"]

#[cfg(feature = "ast")] pub mod ast;

#[cfg(feature = "convert")] pub mod convert;

#[cfg(any(feature = "request", feature = "blocking"))] pub mod request;

#[cfg(feature = "meta")] pub mod meta;

#[cfg(feature = "quick")] pub mod quick;
