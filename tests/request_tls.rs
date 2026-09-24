#![cfg(any(feature = "blocking", feature = "request"))]

use {
  rustls::{
    Certificate, CertificateError, PrivateKey, ServerConfig, ServerConnection,
    StreamOwned,
  },
  std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::TcpListener,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
  },
  url::Url,
};

#[derive(Default)]
struct MemoryCertificates {
  certificates: HashMap<(String, u16), Vec<u8>>,
  reject_save:  bool,
}

impl germ::request::CertificateStore for MemoryCertificates {
  fn load(
    &mut self,
    hostname: &str,
    port: u16,
  ) -> anyhow::Result<Option<Vec<u8>>> {
    Ok(self.certificates.get(&(hostname.to_owned(), port)).cloned())
  }

  fn save(
    &mut self,
    hostname: &str,
    port: u16,
    certificate: &[u8],
  ) -> anyhow::Result<()> {
    anyhow::ensure!(!self.reject_save, "Certificate store rejected save");
    self.certificates.insert((hostname.to_owned(), port), certificate.to_vec());

    Ok(())
  }
}

fn assert_certificate_error(error: &anyhow::Error, expected: CertificateError) {
  let actual = error
    .downcast_ref::<io::Error>()
    .and_then(io::Error::get_ref)
    .and_then(|cause| cause.downcast_ref::<rustls::Error>());

  assert!(
    matches!(actual, Some(rustls::Error::InvalidCertificate(reason)) if reason == &expected),
    "Unexpected TLS error: {error:?}"
  );
}

fn assert_handshake_rejected(server: JoinHandle<io::Result<Vec<u8>>>) {
  let error = server.join().unwrap().unwrap_err();

  assert_ne!(error.kind(), io::ErrorKind::TimedOut);
}

fn assert_no_request_sent(server: JoinHandle<io::Result<Vec<u8>>>) {
  match server.join().unwrap() {
    Ok(bytes) => assert!(bytes.is_empty()),
    Err(error) => assert!(matches!(
      error.kind(),
      io::ErrorKind::UnexpectedEof | io::ErrorKind::ConnectionReset
    )),
  }
}

fn spawn_tls_server(
  certificate_name: &str,
) -> (Url, Certificate, JoinHandle<io::Result<Vec<u8>>>) {
  spawn_tls_server_with_response(
    certificate_name,
    b"20 text/gemini\r\nhello".to_vec(),
    Duration::ZERO,
  )
}

fn spawn_tls_server_with_response(
  certificate_name: &str,
  response_bytes: Vec<u8>,
  response_delay: Duration,
) -> (Url, Certificate, JoinHandle<io::Result<Vec<u8>>>) {
  let certificate =
    rcgen::generate_simple_self_signed(vec![certificate_name.to_owned()])
      .unwrap();
  let certificate_der = Certificate(certificate.serialize_der().unwrap());
  let private_key = PrivateKey(certificate.serialize_private_key_der());
  let config = ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth()
    .with_single_cert(vec![certificate_der.clone()], private_key)
    .unwrap();
  let listener = TcpListener::bind("127.0.0.1:0").unwrap();
  let url = Url::parse(&format!(
    "gemini://localhost:{}/",
    listener.local_addr().unwrap().port()
  ))
  .unwrap();
  let server = thread::spawn(move || {
    let (stream, _) = listener.accept().unwrap();

    stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

    let connection = ServerConnection::new(Arc::new(config)).unwrap();
    let mut tls = StreamOwned::new(connection, stream);
    let mut request_buffer = [0; 1024];
    let bytes_read = tls.read(&mut request_buffer)?;

    if bytes_read > 0 {
      thread::sleep(response_delay);
      tls.write_all(&response_bytes)?;
      tls.conn.send_close_notify();
      tls.flush()?;
    }

    Ok(request_buffer[..bytes_read].to_vec())
  });

  (url, certificate_der, server)
}

fn options_with_certificate(
  certificate: &Certificate,
) -> germ::request::RequestOptions {
  let mut options = germ::request::RequestOptions {
    root_certificates: rustls::RootCertStore::empty(),
    ..Default::default()
  };

  options.root_certificates.add(certificate).unwrap();

  options
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_remembers_first_certificate() {
  let (url, certificate, server) = spawn_tls_server("localhost");
  let mut store = MemoryCertificates::default();
  let response = germ::request::blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .unwrap();

  assert_eq!(response.content().as_deref(), Some("hello"));
  assert_eq!(
    store.certificates.get(&("localhost".to_owned(), url.port().unwrap())),
    Some(&certificate.0)
  );
  assert!(server.join().unwrap().is_ok());
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_accepts_a_previously_pinned_certificate() {
  let (url, certificate, server) = spawn_tls_server("localhost");
  let mut store = MemoryCertificates::default();

  store
    .certificates
    .insert(("localhost".to_owned(), url.port().unwrap()), certificate.0);

  let response = germ::request::blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .unwrap();

  assert_eq!(response.content().as_deref(), Some("hello"));
  assert!(server.join().unwrap().is_ok());
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_rejects_changed_certificate_before_sending_url() {
  let (url, certificate, server) = spawn_tls_server("localhost");
  let different_certificate =
    rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])
      .unwrap()
      .serialize_der()
      .unwrap();
  let mut store = MemoryCertificates::default();

  store.certificates.insert(
    ("localhost".to_owned(), url.port().unwrap()),
    different_certificate.clone(),
  );

  let error = germ::request::blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .unwrap_err();

  assert!(error.to_string().contains("certificate changed"));
  assert_ne!(different_certificate, certificate.0);
  assert_eq!(
    store.certificates.get(&("localhost".to_owned(), url.port().unwrap())),
    Some(&different_certificate)
  );
  assert_no_request_sent(server);
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_rejects_store_failure_before_sending_url() {
  let (url, _, server) = spawn_tls_server("localhost");
  let mut store =
    MemoryCertificates { reject_save: true, ..Default::default() };
  let error = germ::request::blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .unwrap_err();

  assert!(error.to_string().contains("rejected save"));
  assert_no_request_sent(server);
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_rejects_wrong_hostname() {
  let (url, _, server) = spawn_tls_server("elsewhere.test");
  let mut store = MemoryCertificates::default();
  let error = germ::request::blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .unwrap_err();

  assert_certificate_error(&error, CertificateError::NotValidForName);
  assert!(store.certificates.is_empty());
  assert_handshake_rejected(server);
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_tofu_remembers_first_certificate() {
  let (url, certificate, server) = spawn_tls_server("localhost");
  let mut store = MemoryCertificates::default();
  let response = germ::request::non_blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .await
  .unwrap();

  assert_eq!(response.content().as_deref(), Some("hello"));
  assert_eq!(
    store.certificates.get(&("localhost".to_owned(), url.port().unwrap())),
    Some(&certificate.0)
  );
  assert!(server.join().unwrap().is_ok());
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_tofu_rejects_changed_certificate_before_sending_url() {
  let (url, _, server) = spawn_tls_server("localhost");
  let different_certificate =
    rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])
      .unwrap()
      .serialize_der()
      .unwrap();
  let mut store = MemoryCertificates::default();

  store.certificates.insert(
    ("localhost".to_owned(), url.port().unwrap()),
    different_certificate,
  );

  let error = germ::request::non_blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .await
  .unwrap_err();

  assert!(error.to_string().contains("certificate changed"));
  assert_no_request_sent(server);
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_tofu_rejects_store_failure_before_sending_url() {
  let (url, _, server) = spawn_tls_server("localhost");
  let mut store =
    MemoryCertificates { reject_save: true, ..Default::default() };
  let error = germ::request::non_blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .await
  .unwrap_err();

  assert!(error.to_string().contains("rejected save"));
  assert_no_request_sent(server);
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_tofu_rejects_wrong_hostname() {
  let (url, _, server) = spawn_tls_server("elsewhere.test");
  let mut store = MemoryCertificates::default();
  let error = germ::request::non_blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .await
  .unwrap_err();

  assert_certificate_error(&error, CertificateError::NotValidForName);
  assert!(store.certificates.is_empty());
  assert_handshake_rejected(server);
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_request_rejects_untrusted_certificate_before_sending_url() {
  let (url, _, server) = spawn_tls_server("localhost");
  let error = germ::request::blocking::request(&url).unwrap_err();

  assert_certificate_error(&error, CertificateError::UnknownIssuer);
  assert_handshake_rejected(server);
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_request_accepts_explicitly_trusted_certificate() {
  let (mut url, certificate, server) = spawn_tls_server("localhost");
  let mut roots = rustls::RootCertStore::empty();

  url.set_fragment(Some("not-sent"));
  roots.add(&certificate).unwrap();

  let response =
    germ::request::blocking::request_with_roots(&url, &roots).unwrap();

  assert_eq!(response.content().as_deref(), Some("hello"));
  assert_eq!(
    server.join().unwrap().unwrap(),
    format!("gemini://localhost:{}/\r\n", url.port().unwrap()).as_bytes()
  );
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_request_rejects_wrong_hostname() {
  let (url, certificate, server) = spawn_tls_server("elsewhere.test");
  let mut roots = rustls::RootCertStore::empty();

  roots.add(&certificate).unwrap();

  let error =
    germ::request::blocking::request_with_roots(&url, &roots).unwrap_err();

  assert_certificate_error(&error, CertificateError::NotValidForName);
  assert_handshake_rejected(server);
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_request_rejects_oversized_response() {
  let response_bytes = b"20 text/gemini\r\nhello".to_vec();
  let (url, certificate, server) = spawn_tls_server_with_response(
    "localhost",
    response_bytes.clone(),
    Duration::ZERO,
  );
  let mut options = options_with_certificate(&certificate);

  options.max_response_bytes = response_bytes.len() - 1;

  let error =
    germ::request::blocking::request_with_options(&url, &options).unwrap_err();

  assert!(error.to_string().contains("response exceeds"));
  assert!(server.join().is_ok());
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_request_times_out_waiting_for_response() {
  let (url, certificate, server) = spawn_tls_server_with_response(
    "localhost",
    b"20 text/gemini\r\nhello".to_vec(),
    Duration::from_millis(200),
  );
  let mut options = options_with_certificate(&certificate);

  options.timeout = Duration::from_millis(50);

  let error =
    germ::request::blocking::request_with_options(&url, &options).unwrap_err();

  assert_eq!(
    error.downcast_ref::<io::Error>().unwrap().kind(),
    io::ErrorKind::TimedOut
  );
  assert!(server.join().is_ok());
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_request_rejects_malformed_header() {
  let (url, certificate, server) = spawn_tls_server_with_response(
    "localhost",
    b"\xc3\xa9 text/gemini\r\nhello".to_vec(),
    Duration::ZERO,
  );
  let options = options_with_certificate(&certificate);
  let error =
    germ::request::blocking::request_with_options(&url, &options).unwrap_err();

  assert!(error.to_string().contains("two digits and a space"));
  assert!(server.join().is_ok());
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_request_rejects_untrusted_certificate_before_sending_url() {
  let (url, _, server) = spawn_tls_server("localhost");
  let error = germ::request::request(&url).await.unwrap_err();

  assert_certificate_error(&error, CertificateError::UnknownIssuer);
  assert_handshake_rejected(server);
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_request_accepts_explicitly_trusted_certificate() {
  let (mut url, certificate, server) = spawn_tls_server("localhost");
  let mut roots = rustls::RootCertStore::empty();

  url.set_fragment(Some("not-sent"));
  roots.add(&certificate).unwrap();

  let response = germ::request::non_blocking::request_with_roots(&url, &roots)
    .await
    .unwrap();

  assert_eq!(response.content().as_deref(), Some("hello"));
  assert_eq!(
    server.join().unwrap().unwrap(),
    format!("gemini://localhost:{}/\r\n", url.port().unwrap()).as_bytes()
  );
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_request_rejects_wrong_hostname() {
  let (url, certificate, server) = spawn_tls_server("elsewhere.test");
  let mut roots = rustls::RootCertStore::empty();

  roots.add(&certificate).unwrap();

  let error = germ::request::non_blocking::request_with_roots(&url, &roots)
    .await
    .unwrap_err();

  assert_certificate_error(&error, CertificateError::NotValidForName);
  assert_handshake_rejected(server);
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_request_rejects_oversized_response() {
  let response_bytes = b"20 text/gemini\r\nhello".to_vec();
  let (url, certificate, server) = spawn_tls_server_with_response(
    "localhost",
    response_bytes.clone(),
    Duration::ZERO,
  );
  let mut options = options_with_certificate(&certificate);

  options.max_response_bytes = response_bytes.len() - 1;

  let error = germ::request::non_blocking::request_with_options(&url, &options)
    .await
    .unwrap_err();

  assert!(error.to_string().contains("response exceeds"));
  assert!(server.join().is_ok());
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_request_times_out_waiting_for_response() {
  let (url, certificate, server) = spawn_tls_server_with_response(
    "localhost",
    b"20 text/gemini\r\nhello".to_vec(),
    Duration::from_millis(200),
  );
  let mut options = options_with_certificate(&certificate);

  options.timeout = Duration::from_millis(50);

  let error = germ::request::non_blocking::request_with_options(&url, &options)
    .await
    .unwrap_err();

  assert_eq!(error.to_string(), "Gemini request timed out");
  assert!(server.join().is_ok());
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_request_rejects_malformed_header() {
  let (url, certificate, server) = spawn_tls_server_with_response(
    "localhost",
    b"\xc3\xa9 text/gemini\r\nhello".to_vec(),
    Duration::ZERO,
  );
  let options = options_with_certificate(&certificate);
  let error = germ::request::non_blocking::request_with_options(&url, &options)
    .await
    .unwrap_err();

  assert!(error.to_string().contains("two digits and a space"));
  assert!(server.join().is_ok());
}
