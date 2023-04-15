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

//! Make Gemini requests and get sane, structured results

mod response;
mod status;
mod verifier;

use std::io::{Read, Write};

pub use response::Response;
pub use status::Status;
use verifier::GermVerifier;

/// Make a request to a Gemini server. The `url` **should** be prefixed with a
/// scheme (e.g. "gemini://").
///
/// # Example
///
/// ```rust
/// match germ::request::request(&url::Url::parse("gemini://fuwn.me").unwrap()) {
///   Ok(response) => println!("{:?}", response),
///   Err(_) => {}
/// }
/// ```
///
/// # Errors
/// - May error if the URL is invalid
/// - May error if the TLS write fails
/// - May error if the TLS read fails
pub fn request(url: &url::Url) -> anyhow::Result<Response> {
  let config = rustls::ClientConfig::builder()
    .with_safe_defaults()
    .with_custom_certificate_verifier(std::sync::Arc::new(GermVerifier::new()))
    .with_no_client_auth();
  let mut connection = rustls::ClientConnection::new(
    std::sync::Arc::new(config),
    url.domain().unwrap_or("").try_into()?,
  )?;
  let mut stream = std::net::TcpStream::connect(format!(
    "{}:{}",
    url.domain().unwrap_or(""),
    url.port().unwrap_or(1965)
  ))?;
  let mut tls = rustls::Stream::new(&mut connection, &mut stream);

  tls.write_all(format!("{url}\r\n").as_bytes())?;

  let mut plain_text = Vec::new();

  tls.read_to_end(&mut plain_text)?;

  Ok(Response::new(
    &plain_text,
    tls.conn.negotiated_cipher_suite(),
  ))
}
