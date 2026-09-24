use {
  crate::request::{
    RequestOptions, Response, request_line, trust::client_config,
  },
  std::sync::Arc,
  tokio::io::{AsyncReadExt, AsyncWriteExt},
};

/// Make a request to a Gemini server.
///
/// The `url` **should** be prefixed with a scheme (e.g. "gemini://").
///
/// # Example
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///   let url = url::Url::parse("gemini://fuwn.me")?;
///   let response = germ::request::request(&url).await?;
///
///   println!("{response:?}");
///
///   Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns an error for an invalid URL, failed connection or TLS validation,
/// elapsed timeout, oversized response, or malformed response header.
pub async fn request(url: &url::Url) -> anyhow::Result<Response> {
  request_with_options(url, &RequestOptions::default()).await
}

/// Make a request using the supplied trusted certificate authorities.
///
/// The supplied roots replace the default Mozilla roots. To keep those roots,
/// start with `default_root_certificates()` before adding a capsule's DER
/// certificate with `RootCertStore::add`. Added certificates become trust
/// anchors, not exact fingerprint pins. The server name and certificate
/// validity period are still checked. Default time and size limits apply.
///
/// # Errors
///
/// Returns an error for an invalid URL, failed connection or TLS validation,
/// elapsed timeout, oversized response, or malformed response header.
pub async fn request_with_roots(
  url: &url::Url,
  roots: &rustls::RootCertStore,
) -> anyhow::Result<Response> {
  let options = RequestOptions::with_roots(roots.clone());

  request_with_options(url, &options).await
}

/// Make a request with the supplied certificate, time, and size limits.
///
/// The timeout covers connection, TLS, sending, and receiving. The response
/// limit includes the header and rejects an oversized response.
///
/// # Errors
///
/// Returns an error for an invalid URL, failed connection or TLS validation,
/// elapsed timeout, oversized response, or malformed response header.
pub async fn request_with_options(
  url: &url::Url,
  options: &RequestOptions,
) -> anyhow::Result<Response> {
  let request_line = request_line(url)?;
  let domain = url
    .domain()
    .ok_or_else(|| anyhow::anyhow!("Invalid URL: missing domain"))?;
  let server_name = rustls::ServerName::try_from(domain)?;

  tokio::time::timeout(options.timeout, async {
    let address = (domain, url.port().unwrap_or(1965));
    let stream = tokio::net::TcpStream::connect(address).await?;
    let connector = tokio_rustls::TlsConnector::from(Arc::new(client_config(
      options.root_certificates.clone(),
    )));
    let mut tls = connector.connect(server_name, stream).await?;
    let cipher_suite = tls.get_mut().1.negotiated_cipher_suite();

    tls.write_all(request_line.as_bytes()).await?;

    let mut response_bytes = Vec::new();
    let read_limit_bytes = options.max_response_bytes.saturating_add(1) as u64;

    tls.take(read_limit_bytes).read_to_end(&mut response_bytes).await?;
    anyhow::ensure!(
      response_bytes.len() <= options.max_response_bytes,
      "Gemini response exceeds {} bytes",
      options.max_response_bytes
    );

    Response::parse(&response_bytes, cipher_suite)
  })
  .await
  .map_err(|_| anyhow::anyhow!("Gemini request timed out"))?
}
