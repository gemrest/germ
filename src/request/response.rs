// This file is part of Germ <https://github.com/gemrest/germ>.
// Copyright (C) 2022-2025 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2025 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

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
    let header = String::from_utf8_lossy(header_bytes).trim_end().to_string();
    let content_bytes = if header_end < data.len() {
      Some(data[header_end..].to_vec())
    } else {
      None
    };
    let (status_string, meta_string) = header.split_at(2);
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
