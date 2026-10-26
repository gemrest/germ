#![cfg(any(feature = "blocking", feature = "request"))]

use {
  rcgen::{CertificateParams, CertifiedKey, KeyPair},
  rustls::{
    CertificateError, ServerConfig, ServerConnection, StreamOwned,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
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

fn certificate_error(error: &anyhow::Error) -> Option<&CertificateError> {
  match error
    .downcast_ref::<io::Error>()
    .and_then(io::Error::get_ref)
    .and_then(|cause| cause.downcast_ref::<rustls::Error>())
  {
    Some(rustls::Error::InvalidCertificate(reason)) => Some(reason),
    _ => None,
  }
}

fn assert_certificate_error(error: &anyhow::Error, expected: CertificateError) {
  assert_eq!(
    certificate_error(error),
    Some(&expected),
    "Unexpected TLS error: {error:?}"
  );
}

fn assert_not_valid_for_name(error: &anyhow::Error) {
  assert!(
    matches!(
      certificate_error(error),
      Some(CertificateError::NotValidForNameContext { .. })
    ),
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

type TlsServer =
  (Url, CertificateDer<'static>, JoinHandle<io::Result<Vec<u8>>>);

fn spawn_tls_server(certificate_name: &str) -> TlsServer {
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
) -> TlsServer {
  let certificate =
    rcgen::generate_simple_self_signed(vec![certificate_name.to_owned()])
      .unwrap();

  spawn_tls_server_with_configuration(
    server_configuration(&certificate, &certificate.signing_key),
    certificate.cert.der().clone(),
    response_bytes,
    response_delay,
  )
}

fn private_key(key_pair: &KeyPair) -> PrivateKeyDer<'static> {
  PrivatePkcs8KeyDer::from(key_pair.serialize_der()).into()
}

fn server_configuration(
  certificate: &CertifiedKey<KeyPair>,
  signing_key: &KeyPair,
) -> ServerConfig {
  ServerConfig::builder()
    .with_no_client_auth()
    .with_single_cert(
      vec![certificate.cert.der().clone()],
      private_key(signing_key),
    )
    .unwrap()
}

fn self_signed(parameters: CertificateParams) -> CertifiedKey<KeyPair> {
  let signing_key = KeyPair::generate().unwrap();
  let cert = parameters.self_signed(&signing_key).unwrap();

  CertifiedKey { cert, signing_key }
}

fn spawn_tls_server_with_configuration(
  configuration: ServerConfig,
  certificate: CertificateDer<'static>,
  response_bytes: Vec<u8>,
  response_delay: Duration,
) -> TlsServer {
  let listener = TcpListener::bind("127.0.0.1:0").unwrap();
  let url = Url::parse(&format!(
    "gemini://localhost:{}/",
    listener.local_addr().unwrap().port()
  ))
  .unwrap();
  let server = thread::spawn(move || {
    let (stream, _) = listener.accept().unwrap();

    stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

    let connection = ServerConnection::new(Arc::new(configuration)).unwrap();
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

  (url, certificate, server)
}

fn unnamed_ca_certificate() -> CertifiedKey<KeyPair> {
  let mut parameters = CertificateParams::default();

  parameters.distinguished_name = rcgen::DistinguishedName::new();
  parameters.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

  self_signed(parameters)
}

fn spawn_certificate_server(certificate: CertifiedKey<KeyPair>) -> TlsServer {
  spawn_tls_server_with_configuration(
    server_configuration(&certificate, &certificate.signing_key),
    certificate.cert.der().clone(),
    b"20 text/gemini\r\nhello".to_vec(),
    Duration::ZERO,
  )
}

fn dated_certificate(
  not_before_year: i32,
  not_after_year: i32,
) -> CertifiedKey<KeyPair> {
  let mut parameters =
    CertificateParams::new(vec!["elsewhere.test".to_owned()]).unwrap();

  parameters.not_before = rcgen::date_time_ymd(not_before_year, 1, 1);
  parameters.not_after = rcgen::date_time_ymd(not_after_year, 1, 1);
  parameters.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

  self_signed(parameters)
}

fn options_with_certificate(
  certificate: &CertificateDer<'static>,
) -> germ::request::RequestOptions {
  let mut options = germ::request::RequestOptions {
    root_certificates: rustls::RootCertStore::empty(),
    ..Default::default()
  };

  options.root_certificates.add(certificate.clone()).unwrap();

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
    Some(&certificate.to_vec())
  );
  assert!(server.join().unwrap().is_ok());
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_accepts_ca_certificate_without_names() {
  let (url, certificate, server) =
    spawn_certificate_server(unnamed_ca_certificate());
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
    Some(&certificate.to_vec())
  );
  assert!(server.join().unwrap().is_ok());
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_pins_certificates_regardless_of_validity_dates() {
  for (not_before_year, not_after_year) in [(2018, 2019), (3000, 3001)] {
    let (url, certificate, server) = spawn_certificate_server(
      dated_certificate(not_before_year, not_after_year),
    );
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
      Some(&certificate.to_vec())
    );
    assert!(server.join().unwrap().is_ok());
  }
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_replaces_expired_pin_after_tls_signature_verification() {
  let previous_certificate = dated_certificate(2018, 2019).cert.der().to_vec();
  let (url, certificate, server) =
    spawn_certificate_server(unnamed_ca_certificate());
  let mut store = MemoryCertificates::default();

  store.certificates.insert(
    ("localhost".to_owned(), url.port().unwrap()),
    previous_certificate,
  );

  let response = germ::request::blocking::request_with_tofu(
    &url,
    &mut store,
    &Default::default(),
  )
  .unwrap();

  assert_eq!(response.content().as_deref(), Some("hello"));
  assert_eq!(
    store.certificates.get(&("localhost".to_owned(), url.port().unwrap())),
    Some(&certificate.to_vec())
  );
  assert!(server.join().unwrap().is_ok());
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_rejects_invalid_handshake_signatures() {
  for version in [&rustls::version::TLS12, &rustls::version::TLS13] {
    let certificate = unnamed_ca_certificate();
    let other_certificate = unnamed_ca_certificate();
    let mismatched_key = rustls::sign::CertifiedKey::new(
      vec![certificate.cert.der().clone()],
      rustls::crypto::ring::sign::any_supported_type(&private_key(
        &other_certificate.signing_key,
      ))
      .unwrap(),
    );
    let configuration =
      ServerConfig::builder_with_protocol_versions(&[version])
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(rustls::sign::SingleCertAndKey::from(
          mismatched_key,
        )));
    let (url, _, server) = spawn_tls_server_with_configuration(
      configuration,
      certificate.cert.der().clone(),
      b"20 text/gemini\r\nhello".to_vec(),
      Duration::ZERO,
    );
    let mut store = MemoryCertificates::default();
    let error = germ::request::blocking::request_with_tofu(
      &url,
      &mut store,
      &Default::default(),
    )
    .unwrap_err();

    assert_certificate_error(&error, CertificateError::BadSignature);
    assert!(store.certificates.is_empty());
    assert_handshake_rejected(server);
  }
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_tofu_accepts_a_previously_pinned_certificate() {
  let (url, certificate, server) = spawn_tls_server("localhost");
  let mut store = MemoryCertificates::default();

  store.certificates.insert(
    ("localhost".to_owned(), url.port().unwrap()),
    certificate.to_vec(),
  );

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
      .cert
      .der()
      .to_vec();
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
  assert_ne!(different_certificate, certificate.to_vec());
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
fn blocking_tofu_pins_certificate_with_unrelated_hostname() {
  let (url, certificate, server) = spawn_tls_server("elsewhere.test");
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
    Some(&certificate.to_vec())
  );
  assert!(server.join().unwrap().is_ok());
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
    Some(&certificate.to_vec())
  );
  assert!(server.join().unwrap().is_ok());
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_tofu_accepts_ca_certificate_without_names() {
  let (url, certificate, server) =
    spawn_certificate_server(unnamed_ca_certificate());
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
    Some(&certificate.to_vec())
  );
  assert!(server.join().unwrap().is_ok());
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_tofu_pins_certificates_regardless_of_validity_dates() {
  for (not_before_year, not_after_year) in [(2018, 2019), (3000, 3001)] {
    let (url, certificate, server) = spawn_certificate_server(
      dated_certificate(not_before_year, not_after_year),
    );
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
      Some(&certificate.to_vec())
    );
    assert!(server.join().unwrap().is_ok());
  }
}

#[cfg(feature = "request")]
#[tokio::test]
async fn async_tofu_replaces_expired_pin_after_tls_signature_verification() {
  let previous_certificate = dated_certificate(2018, 2019).cert.der().to_vec();
  let (url, certificate, server) =
    spawn_certificate_server(unnamed_ca_certificate());
  let mut store = MemoryCertificates::default();

  store.certificates.insert(
    ("localhost".to_owned(), url.port().unwrap()),
    previous_certificate,
  );

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
    Some(&certificate.to_vec())
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
      .cert
      .der()
      .to_vec();
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
async fn async_tofu_pins_certificate_with_unrelated_hostname() {
  let (url, certificate, server) = spawn_tls_server("elsewhere.test");
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
    Some(&certificate.to_vec())
  );
  assert!(server.join().unwrap().is_ok());
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
  roots.add(certificate).unwrap();

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

  roots.add(certificate).unwrap();

  let error =
    germ::request::blocking::request_with_roots(&url, &roots).unwrap_err();

  assert_not_valid_for_name(&error);
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
  roots.add(certificate).unwrap();

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

  roots.add(certificate).unwrap();

  let error = germ::request::non_blocking::request_with_roots(&url, &roots)
    .await
    .unwrap_err();

  assert_not_valid_for_name(&error);
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
