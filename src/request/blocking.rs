use {
  crate::request::{GermVerifier, Response},
  std::io::{Read, Write},
};

/// Make a request to a Gemini server. The `url` **should** be prefixed with a
/// scheme (e.g. "gemini://").
///
/// # Example
///
/// ```rust
/// match germ::request::blocking::request(
///   &url::Url::parse("gemini://fuwn.me").unwrap(),
/// ) {
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
  let domain = url
    .domain()
    .ok_or_else(|| anyhow::anyhow!("Invalid URL: missing domain"))?;
  let mut connection = rustls::ClientConnection::new(
    std::sync::Arc::new(config),
    domain.try_into()?,
  )?;
  let mut stream = std::net::TcpStream::connect(format!(
    "{}:{}",
    domain,
    url.port().unwrap_or(1965)
  ))?;
  let mut tls = rustls::Stream::new(&mut connection, &mut stream);

  tls.write_all(format!("{url}\r\n").as_bytes())?;

  let mut plain_text = Vec::new();

  tls.read_to_end(&mut plain_text)?;

  Ok(Response::new(&plain_text, tls.conn.negotiated_cipher_suite()))
}
