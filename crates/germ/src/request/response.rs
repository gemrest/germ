// This file is part of Germ <https://github.com/gemrest/germ>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
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
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::borrow::Cow;

use rustls::SupportedCipherSuite;

use crate::request::Status;

#[derive(Debug, Clone)]
pub struct Response {
  status:  Status,
  meta:    String,
  content: Option<String>,
  size:    usize,
  suite:   Option<SupportedCipherSuite>,
}

impl Response {
  pub(super) fn new(data: &[u8], suite: Option<SupportedCipherSuite>) -> Self {
    let string_form = String::from_utf8_lossy(data).to_string();
    let mut content = None;
    let header;

    if string_form.ends_with("\r\n") {
      header = string_form;
    } else {
      let mut string_split = string_form.split("\r\n");

      header = string_split.next().unwrap_or("").to_string();
      content = Some(string_split.map(|s| format!("{s}\r\n")).collect());
    }

    let header_split = header.split_at(2);

    Self {
      status: Status::from(header_split.0.parse::<i32>().unwrap_or(0)),
      meta: header_split.1.trim_start().to_string(),
      content,
      size: data.len(),
      suite,
    }
  }

  #[must_use]
  pub const fn status(&self) -> &Status { &self.status }

  #[must_use]
  pub fn meta(&self) -> Cow<'_, str> { Cow::Borrowed(&self.meta) }

  #[must_use]
  pub const fn content(&self) -> &Option<String> { &self.content }

  #[must_use]
  pub const fn size(&self) -> &usize { &self.size }

  #[must_use]
  pub const fn suite(&self) -> &Option<SupportedCipherSuite> { &self.suite }
}
