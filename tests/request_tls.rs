#![cfg(any(feature = "blocking", feature = "request"))]

use {
  rustls::{
    Certificate, CertificateError, PrivateKey, ServerConfig, ServerConnection,
    StreamOwned,
  },
  std::{
    io::{self, Read, Write},
    net::TcpListener,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
  },
  url::Url,
};

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

fn spawn_tls_server(
  certificate_name: &str,
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
      tls.write_all(b"20 text/gemini\r\nhello").unwrap();
      tls.conn.send_close_notify();
      tls.flush().unwrap();
    }

    Ok(request_buffer[..bytes_read].to_vec())
  });

  (url, certificate_der, server)
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
  let (url, certificate, server) = spawn_tls_server("localhost");
  let mut roots = rustls::RootCertStore::empty();

  roots.add(&certificate).unwrap();

  let response =
    germ::request::blocking::request_with_roots(&url, &roots).unwrap();

  assert_eq!(response.content().as_deref(), Some("hello"));
  assert_eq!(server.join().unwrap().unwrap(), format!("{url}\r\n").as_bytes());
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
  let (url, certificate, server) = spawn_tls_server("localhost");
  let mut roots = rustls::RootCertStore::empty();

  roots.add(&certificate).unwrap();

  let response = germ::request::non_blocking::request_with_roots(&url, &roots)
    .await
    .unwrap();

  assert_eq!(response.content().as_deref(), Some("hello"));
  assert_eq!(server.join().unwrap().unwrap(), format!("{url}\r\n").as_bytes());
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
