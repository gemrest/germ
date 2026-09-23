use {
  crate::request::{Response, default_root_certificates, trust::client_config},
  std::sync::Arc,
  tokio::io::{AsyncReadExt, AsyncWriteExt},
};

/// Make a request to a Gemini server.
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
/// Returns an error if the URL is invalid, the server is unreachable, its
/// certificate is invalid or untrusted, or TLS I/O fails.
pub async fn request(url: &url::Url) -> anyhow::Result<Response> {
  request_with_roots(url, &default_root_certificates()).await
}

/// Make a request using the supplied trusted certificate authorities.
///
/// The supplied roots replace the default Mozilla roots. To keep those roots,
/// start with `default_root_certificates()` before adding a capsule's DER
/// certificate with `RootCertStore::add`. Added certificates become trust
/// anchors, not exact fingerprint pins. The server name and certificate
/// validity period are still checked.
///
/// # Errors
///
/// Returns an error if the URL is invalid, the server is unreachable, its
/// certificate is invalid or untrusted, or TLS I/O fails.
pub async fn request_with_roots(
  url: &url::Url,
  roots: &rustls::RootCertStore,
) -> anyhow::Result<Response> {
  let domain = url
    .domain()
    .ok_or_else(|| anyhow::anyhow!("Invalid URL: missing domain"))?;
  let server_name = rustls::ServerName::try_from(domain)?;
  let address = format!("{}:{}", domain, url.port().unwrap_or(1965));
  let stream = tokio::net::TcpStream::connect(address).await?;
  let connector =
    tokio_rustls::TlsConnector::from(Arc::new(client_config(roots.clone())));
  let mut tls = connector.connect(server_name, stream).await?;
  let cipher_suite = tls.get_mut().1.negotiated_cipher_suite();

  tls.write_all(format!("{url}\r\n").as_bytes()).await?;

  let mut response_bytes = Vec::new();

  tls.read_to_end(&mut response_bytes).await?;

  Ok(Response::new(&response_bytes, cipher_suite))
}
