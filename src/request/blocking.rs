use {
  crate::request::{
    RequestOptions, Response, request_line, trust::client_config,
  },
  std::{
    io::{self, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    sync::Arc,
    time::{Duration, Instant},
  },
};

// Recompute the timeout before each TLS read and write so partial I/O shares
// one request deadline.
struct DeadlineStream {
  stream:   TcpStream,
  deadline: Instant,
}

impl Read for DeadlineStream {
  fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
    let remaining_duration = remaining_time(self.deadline)?;

    self.stream.set_read_timeout(Some(remaining_duration))?;

    self.stream.read(buffer).map_err(normalize_timeout)
  }
}

impl Write for DeadlineStream {
  fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
    let remaining_duration = remaining_time(self.deadline)?;

    self.stream.set_write_timeout(Some(remaining_duration))?;

    self.stream.write(buffer).map_err(normalize_timeout)
  }

  fn flush(&mut self) -> io::Result<()> { self.stream.flush() }
}

fn remaining_time(deadline: Instant) -> io::Result<Duration> {
  deadline
    .checked_duration_since(Instant::now())
    .filter(|duration| !duration.is_zero())
    .ok_or_else(timeout_error)
}

fn timeout_error() -> io::Error {
  io::Error::new(io::ErrorKind::TimedOut, "Gemini request timed out")
}

fn normalize_timeout(error: io::Error) -> io::Error {
  if matches!(error.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut)
  {
    timeout_error()
  } else {
    error
  }
}

/// Make a request to a Gemini server. The `url` **should** be prefixed with a
/// scheme (e.g. "gemini://").
///
/// # Example
///
/// ```rust,no_run
/// fn main() -> anyhow::Result<()> {
///   let url = url::Url::parse("gemini://fuwn.me")?;
///   let response = germ::request::blocking::request(&url)?;
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
pub fn request(url: &url::Url) -> anyhow::Result<Response> {
  request_with_options(url, &RequestOptions::default())
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
pub fn request_with_roots(
  url: &url::Url,
  roots: &rustls::RootCertStore,
) -> anyhow::Result<Response> {
  let options = RequestOptions::with_roots(roots.clone());

  request_with_options(url, &options)
}

/// Make a request with the supplied certificate, time, and size limits.
///
/// The timeout covers connection, TLS, sending, and receiving. Hostname lookup
/// uses the system resolver and may outlast the timeout before returning.
/// The response limit includes the header and rejects an oversized response.
///
/// # Errors
///
/// Returns an error for an invalid URL, failed connection or TLS validation,
/// elapsed timeout, oversized response, or malformed response header.
pub fn request_with_options(
  url: &url::Url,
  options: &RequestOptions,
) -> anyhow::Result<Response> {
  let request_line = request_line(url)?;
  let domain = url
    .domain()
    .ok_or_else(|| anyhow::anyhow!("Invalid URL: missing domain"))?;
  let deadline = Instant::now()
    .checked_add(options.timeout)
    .ok_or_else(|| anyhow::anyhow!("Gemini request timeout is too large"))?;
  let mut connection_error =
    io::Error::new(io::ErrorKind::AddrNotAvailable, "No server address found");
  let mut connected_stream = None;

  for address in (domain, url.port().unwrap_or(1965)).to_socket_addrs()? {
    match TcpStream::connect_timeout(&address, remaining_time(deadline)?) {
      Ok(stream) => {
        connected_stream = Some(stream);

        break;
      }
      Err(error) => connection_error = error,
    }
  }

  let mut stream = DeadlineStream {
    stream: connected_stream.ok_or(connection_error)?,
    deadline,
  };
  let mut connection = rustls::ClientConnection::new(
    Arc::new(client_config(options.root_certificates.clone())),
    domain.try_into()?,
  )?;
  let mut tls = rustls::Stream::new(&mut connection, &mut stream);

  tls.write_all(request_line.as_bytes())?;

  let mut response_bytes = Vec::new();
  let read_limit_bytes = options.max_response_bytes.saturating_add(1) as u64;

  Read::take(&mut tls, read_limit_bytes).read_to_end(&mut response_bytes)?;
  anyhow::ensure!(
    response_bytes.len() <= options.max_response_bytes,
    "Gemini response exceeds {} bytes",
    options.max_response_bytes
  );

  Response::parse(&response_bytes, tls.conn.negotiated_cipher_suite())
}
