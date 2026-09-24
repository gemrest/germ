use {crate::request::Status, rustls::SupportedCipherSuite, std::borrow::Cow};

const MAX_META_BYTES: usize = 1024;

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
  status:  Status,
  meta:    String,
  content: Option<Vec<u8>>,
  size:    usize,
  suite:   Option<SupportedCipherSuite>,
}

impl Response {
  pub(crate) fn parse(
    response_bytes: &[u8],
    cipher_suite: Option<SupportedCipherSuite>,
  ) -> anyhow::Result<Self> {
    let delimiter = b"\r\n";
    let header_end = response_bytes
      .windows(delimiter.len())
      .position(|window| window == delimiter)
      .ok_or_else(|| anyhow::anyhow!("Gemini response header has no CRLF"))?;
    let header_bytes = &response_bytes[..header_end];

    anyhow::ensure!(
      header_bytes.len() >= 3
        && header_bytes[0].is_ascii_digit()
        && header_bytes[1].is_ascii_digit()
        && header_bytes[2] == b' ',
      "Gemini response header must begin with two digits and a space"
    );

    let meta_bytes = &header_bytes[3..];

    anyhow::ensure!(
      meta_bytes.len() <= MAX_META_BYTES,
      "Gemini response meta exceeds {MAX_META_BYTES} bytes"
    );
    anyhow::ensure!(
      !meta_bytes.starts_with(&[0xef, 0xbb, 0xbf]),
      "Gemini response meta begins with a byte order mark"
    );
    anyhow::ensure!(
      !meta_bytes.contains(&b'\r') && !meta_bytes.contains(&b'\n'),
      "Gemini response meta contains a line break"
    );

    let meta = std::str::from_utf8(meta_bytes)?.to_owned();
    let status_code = i32::from(header_bytes[0] - b'0') * 10
      + i32::from(header_bytes[1] - b'0');
    let body_start = header_end + delimiter.len();
    let content = (body_start < response_bytes.len())
      .then(|| response_bytes[body_start..].to_vec());

    Ok(Self {
      status: Status::from(status_code),
      meta,
      content,
      size: response_bytes.len(),
      suite: cipher_suite,
    })
  }

  #[must_use]
  pub const fn status(&self) -> &Status { &self.status }

  #[allow(clippy::missing_const_for_fn)]
  #[must_use]
  pub fn meta(&self) -> Cow<'_, str> { Cow::Borrowed(&self.meta) }

  /// Return the body as text, replacing invalid UTF-8 sequences.
  ///
  /// To inspect the original bytes, use
  /// [`Response::content_bytes`].
  #[must_use]
  pub fn content(&self) -> Option<String> {
    self
      .content
      .as_ref()
      .map(|content| String::from_utf8_lossy(content).to_string())
  }

  #[must_use]
  pub fn content_bytes(&self) -> Option<&[u8]> { self.content.as_deref() }

  #[must_use]
  pub const fn size(&self) -> &usize { &self.size }

  #[must_use]
  pub const fn suite(&self) -> &Option<SupportedCipherSuite> { &self.suite }
}

#[cfg(test)]
mod tests {
  use super::{MAX_META_BYTES, Response};

  #[test]
  fn parses_header_and_body_without_changing_meta() {
    let response = Response::parse(b"20  text/gemini\r\nhello", None).unwrap();

    assert_eq!(response.meta(), " text/gemini");
    assert_eq!(response.content_bytes(), Some(b"hello".as_slice()));
    assert_eq!(*response.size(), 22);
  }

  #[test]
  fn rejects_malformed_headers() {
    for header in [
      b"\xc3\xa9 text/gemini\r\n".as_slice(),
      b"2x text/gemini\r\n",
      b"20text/gemini\r\n",
      b"20 text/gemini",
      b"20 bad\nmeta\r\n",
      b"20 \xff\r\n",
      b"20 \xef\xbb\xbfmeta\r\n",
    ] {
      assert!(Response::parse(header, None).is_err(), "{header:?}");
    }
  }

  #[test]
  fn enforces_meta_byte_limit() {
    let accepted = format!("20 {}\r\n", "a".repeat(MAX_META_BYTES));
    let rejected = format!("20 {}\r\n", "a".repeat(MAX_META_BYTES + 1));

    assert!(Response::parse(accepted.as_bytes(), None).is_ok());
    assert!(Response::parse(rejected.as_bytes(), None).is_err());
  }
}
