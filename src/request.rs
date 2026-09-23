//! Make Gemini requests and get sane, structured results

mod response;
mod status;
mod trust;

#[cfg(feature = "blocking")] pub mod blocking;

#[cfg(feature = "request")] pub mod non_blocking;

#[cfg(feature = "request")] pub use non_blocking::request;
pub use {
  response::Response, status::Status, trust::default_root_certificates,
};
