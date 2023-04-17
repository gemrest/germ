// This file is part of Germ <https://github.com/gemrest/germ>.
// Copyright (C) 2022-2023 Fuwn <contact@fuwn.me>
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

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::request::Response;

/// Make a request to a Gemini server
///
/// The `url` **should** be prefixed with a scheme (e.g. "gemini://").
///
/// # Example
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///   match germ::request::sync::request(
///     &url::Url::parse("gemini://fuwn.me").unwrap(),
///   )
///   .await
///   {
///     Ok(response) => println!("{:?}", response),
///     Err(_) => {}
///   }
/// }
/// ```
///
/// # Errors
///
/// - May error if the URL is invalid
/// - May error if the server is unreachable
/// - May error if the TLS write fails
/// - May error if the TLS read fails
pub async fn request(url: &url::Url) -> anyhow::Result<Response> {
  let mut tls = tokio_rustls::TlsConnector::from(std::sync::Arc::new(
    rustls::ClientConfig::builder()
      .with_safe_defaults()
      .with_custom_certificate_verifier(std::sync::Arc::new(
        crate::request::GermVerifier::new(),
      ))
      .with_no_client_auth(),
  ))
  .connect(
    rustls::ServerName::try_from(url.domain().unwrap_or_default())?,
    tokio::net::TcpStream::connect(format!(
      "{}:{}",
      url.domain().unwrap_or(""),
      url.port().unwrap_or(1965)
    ))
    .await?,
  )
  .await?;
  let cipher_suite = tls.get_mut().1.negotiated_cipher_suite();

  tls.write_all(format!("{url}\r\n").as_bytes()).await?;

  Ok(Response::new(
    &{
      let mut plain_text = Vec::new();

      tls.read_to_end(&mut plain_text).await?;

      plain_text
    },
    cipher_suite,
  ))
}
