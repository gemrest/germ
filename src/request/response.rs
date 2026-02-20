use {crate::request::Status, rustls::SupportedCipherSuite, std::borrow::Cow};

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
  status:  Status,
  meta:    String,
  content: Option<Vec<u8>>,
  size:    usize,
  suite:   Option<SupportedCipherSuite>,
}

impl Response {
  pub(crate) fn new(data: &[u8], suite: Option<SupportedCipherSuite>) -> Self {
    let delimiter = b"\r\n";
    let header_end = data
      .windows(delimiter.len())
      .position(|window| window == delimiter)
      .map_or(data.len(), |pos| pos + delimiter.len());
    let header_bytes = &data[..header_end];
    let header_cow = String::from_utf8_lossy(header_bytes);
    let header_trimmed = header_cow.trim_end();
    let content_bytes = if header_end < data.len() {
      Some(data[header_end..].to_vec())
    } else {
      None
    };
    let (status_string, meta_string) = if header_trimmed.len() >= 2 {
      header_trimmed.split_at(2)
    } else {
      (header_trimmed, "")
    };
    let status_code = status_string.parse::<i32>().unwrap_or(0);

    Self {
      status: Status::from(status_code),
      meta: meta_string.trim_start().to_string(),
      content: content_bytes,
      size: data.len(),
      suite,
    }
  }

  #[must_use]
  pub const fn status(&self) -> &Status { &self.status }

  #[allow(clippy::missing_const_for_fn)]
  #[must_use]
  pub fn meta(&self) -> Cow<'_, str> { Cow::Borrowed(&self.meta) }

  /// This associated function assumes that the content is valid UTF-8.
  ///
  /// If you want to handle data bytes directly, use
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
