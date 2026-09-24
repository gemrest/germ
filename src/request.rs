//! Make Gemini requests and get structured results.

use std::time::Duration;

const MAX_REQUEST_URL_BYTES: usize = 1024;
const DEFAULT_GEMINI_PORT: u16 = 1965;

mod response;
mod status;
mod trust;

#[cfg(feature = "blocking")] pub mod blocking;

#[cfg(feature = "request")] pub mod non_blocking;

#[cfg(feature = "request")] pub use non_blocking::request;
pub use {
  response::Response,
  status::{Status, StatusCategory},
  trust::{CertificateStore, default_root_certificates},
};

/// Configures time and size limits for all requests and roots for CA trust.
#[derive(Clone, Debug)]
pub struct RequestOptions {
  /// Sets trusted certificate authorities for CA-verified requests.
  /// Defaults to Mozilla roots; TOFU requests ignore this field.
  pub root_certificates:  rustls::RootCertStore,
  /// Limit elapsed time for the request; defaults to 30 seconds.
  pub timeout:            Duration,
  /// Limit response bytes, including the header; defaults to 16 MiB.
  pub max_response_bytes: usize,
}

impl RequestOptions {
  const fn with_roots(root_certificates: rustls::RootCertStore) -> Self {
    Self {
      root_certificates,
      timeout: Duration::from_secs(30),
      max_response_bytes: 16 * 1024 * 1024,
    }
  }
}

impl Default for RequestOptions {
  fn default() -> Self { Self::with_roots(default_root_certificates()) }
}

pub(crate) fn request_line(url: &url::Url) -> anyhow::Result<String> {
  if url.scheme() == "gemini" {
    anyhow::ensure!(
      url.username().is_empty() && url.password().is_none(),
      "Gemini URLs cannot contain user information"
    );
  }

  let mut request_url = url.clone();

  request_url.set_fragment(None);

  let url_bytes = request_url.as_str().as_bytes();

  anyhow::ensure!(
    url_bytes.len() <= MAX_REQUEST_URL_BYTES,
    "Gemini URL exceeds {MAX_REQUEST_URL_BYTES} bytes"
  );
  anyhow::ensure!(
    !url_bytes.contains(&b'\r') && !url_bytes.contains(&b'\n'),
    "Gemini URL contains a line break"
  );

  Ok(format!("{request_url}\r\n"))
}

#[cfg(test)]
mod tests {
  use {
    super::{MAX_REQUEST_URL_BYTES, request_line},
    url::Url,
  };

  #[test]
  fn request_line_omits_fragment() {
    let url = Url::parse("gemini://example.com/path?query#fragment").unwrap();

    assert_eq!(
      request_line(&url).unwrap(),
      "gemini://example.com/path?query\r\n"
    );
  }

  #[test]
  fn request_line_enforces_url_byte_limit() {
    let prefix = "gemini://example.com/";
    let accepted = Url::parse(&format!(
      "{prefix}{}",
      "a".repeat(MAX_REQUEST_URL_BYTES - prefix.len())
    ))
    .unwrap();
    let rejected = Url::parse(&format!(
      "{prefix}{}",
      "a".repeat(MAX_REQUEST_URL_BYTES + 1 - prefix.len())
    ))
    .unwrap();

    assert_eq!(
      request_line(&accepted).unwrap().len(),
      MAX_REQUEST_URL_BYTES + b"\r\n".len()
    );
    assert!(request_line(&rejected).is_err());
  }

  #[test]
  fn request_line_counts_encoded_unicode_bytes() {
    let prefix = "gemini://example.com/";
    let encoded_character_bytes = "%C3%A9".len();
    let accepted = Url::parse(&format!(
      "{prefix}{}é",
      "a"
        .repeat(MAX_REQUEST_URL_BYTES - prefix.len() - encoded_character_bytes)
    ))
    .unwrap();
    let rejected = Url::parse(&format!(
      "{prefix}{}é",
      "a".repeat(
        MAX_REQUEST_URL_BYTES + 1 - prefix.len() - encoded_character_bytes
      )
    ))
    .unwrap();

    assert_eq!(
      request_line(&accepted).unwrap().len(),
      MAX_REQUEST_URL_BYTES + b"\r\n".len()
    );
    assert!(request_line(&rejected).is_err());
  }

  #[test]
  fn request_line_rejects_user_information() {
    let url = Url::parse("gemini://user:secret@example.com/").unwrap();

    assert!(request_line(&url).is_err());
  }
}
