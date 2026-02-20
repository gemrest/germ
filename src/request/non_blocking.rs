use {
  crate::request::Response,
  tokio::io::{AsyncReadExt, AsyncWriteExt},
};

/// Make a request to a Gemini server
///
/// The `url` **should** be prefixed with a scheme (e.g. "gemini://").
///
/// # Example
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///   match germ::request::request(&url::Url::parse("gemini://fuwn.me").unwrap())
///     .await
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
    rustls::ServerName::try_from(
      url
        .domain()
        .ok_or_else(|| anyhow::anyhow!("Invalid URL: missing domain"))?,
    )?,
    tokio::net::TcpStream::connect(format!(
      "{}:{}",
      url
        .domain()
        .ok_or_else(|| anyhow::anyhow!("Invalid URL: missing domain"))?,
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
