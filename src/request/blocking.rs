use {
  crate::request::{Response, default_root_certificates, trust::client_config},
  std::{
    io::{Read, Write},
    sync::Arc,
  },
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
///
/// Returns an error if the URL is invalid, the server certificate is invalid
/// or untrusted, or TLS I/O fails.
pub fn request(url: &url::Url) -> anyhow::Result<Response> {
  request_with_roots(url, &default_root_certificates())
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
/// Returns an error if the URL is invalid, the server certificate is invalid
/// or untrusted, or TLS I/O fails.
pub fn request_with_roots(
  url: &url::Url,
  roots: &rustls::RootCertStore,
) -> anyhow::Result<Response> {
  let domain = url
    .domain()
    .ok_or_else(|| anyhow::anyhow!("Invalid URL: missing domain"))?;
  let mut connection = rustls::ClientConnection::new(
    Arc::new(client_config(roots.clone())),
    domain.try_into()?,
  )?;
  let mut stream = std::net::TcpStream::connect(format!(
    "{}:{}",
    domain,
    url.port().unwrap_or(1965)
  ))?;
  let mut tls = rustls::Stream::new(&mut connection, &mut stream);

  tls.write_all(format!("{url}\r\n").as_bytes())?;

  let mut response_bytes = Vec::new();

  tls.read_to_end(&mut response_bytes)?;

  Ok(Response::new(&response_bytes, tls.conn.negotiated_cipher_suite()))
}
