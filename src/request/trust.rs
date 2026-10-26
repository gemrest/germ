use {
  rustls::{
    CertificateError, ClientConfig, DigitallySignedStruct, Error,
    RootCertStore, SignatureScheme,
    client::danger::{
      HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
    },
    crypto::{CryptoProvider, WebPkiSupportedAlgorithms, ring},
    pki_types::{CertificateDer, ServerName, UnixTime},
  },
  std::{sync::Arc, time::SystemTime},
  x509_cert::der::Decode,
};

/// Stores exact server certificates by hostname and port across requests.
///
/// Implementations should persist updates before returning from `save` and
/// coordinate concurrent writers to avoid silently replacing a changed pin.
/// Store methods run synchronously, including during async requests.
pub trait CertificateStore: Send {
  /// Loads the previously trusted DER certificate, if any.
  ///
  /// # Errors
  ///
  /// Returns an error when the store cannot read the certificate.
  fn load(
    &mut self,
    hostname: &str,
    port: u16,
  ) -> anyhow::Result<Option<Vec<u8>>>;

  /// Saves the DER certificate as the trusted certificate for this server.
  ///
  /// # Errors
  ///
  /// Returns an error when the store cannot persist the certificate.
  fn save(
    &mut self,
    hostname: &str,
    port: u16,
    certificate: &[u8],
  ) -> anyhow::Result<()>;
}

#[derive(Debug)]
struct TrustOnFirstUseVerifier {
  signature_algorithms: WebPkiSupportedAlgorithms,
}

impl TrustOnFirstUseVerifier {
  fn new() -> Self {
    Self {
      signature_algorithms: crypto_provider().signature_verification_algorithms,
    }
  }
}

impl ServerCertVerifier for TrustOnFirstUseVerifier {
  fn verify_server_cert(
    &self,
    end_entity: &CertificateDer<'_>,
    _intermediates: &[CertificateDer<'_>],
    _server_name: &ServerName<'_>,
    _ocsp_response: &[u8],
    _now: UnixTime,
  ) -> Result<ServerCertVerified, Error> {
    x509_cert::Certificate::from_der(end_entity)
      .map_err(|_| Error::InvalidCertificate(CertificateError::BadEncoding))?;

    // TOFU pins the exact certificate before sending the URL. Its expiry
    // controls pin replacement; the signature methods verify the handshake.
    Ok(ServerCertVerified::assertion())
  }

  fn verify_tls12_signature(
    &self,
    message: &[u8],
    certificate: &CertificateDer<'_>,
    signature: &DigitallySignedStruct,
  ) -> Result<HandshakeSignatureValid, Error> {
    rustls::crypto::verify_tls12_signature(
      message,
      certificate,
      signature,
      &self.signature_algorithms,
    )
  }

  fn verify_tls13_signature(
    &self,
    message: &[u8],
    certificate: &CertificateDer<'_>,
    signature: &DigitallySignedStruct,
  ) -> Result<HandshakeSignatureValid, Error> {
    rustls::crypto::verify_tls13_signature(
      message,
      certificate,
      signature,
      &self.signature_algorithms,
    )
  }

  fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
    self.signature_algorithms.supported_schemes()
  }
}

// Select the provider explicitly so dependents enabling another rustls
// provider cannot make the process-wide default ambiguous.
fn crypto_provider() -> CryptoProvider { ring::default_provider() }

fn client_config_builder()
-> Result<rustls::ConfigBuilder<ClientConfig, rustls::WantsVerifier>, Error> {
  ClientConfig::builder_with_provider(Arc::new(crypto_provider()))
    .with_safe_default_protocol_versions()
}

pub fn tofu_client_config() -> Result<ClientConfig, Error> {
  Ok(
    client_config_builder()?
      .dangerous()
      .with_custom_certificate_verifier(
        Arc::new(TrustOnFirstUseVerifier::new()),
      )
      .with_no_client_auth(),
  )
}

pub fn check_tofu(
  store: &mut dyn CertificateStore,
  hostname: &str,
  port: u16,
  certificate: Option<&CertificateDer<'_>>,
) -> anyhow::Result<()> {
  let certificate = certificate.ok_or_else(|| {
    anyhow::anyhow!("Gemini server did not provide a certificate")
  })?;

  match store.load(hostname, port)? {
    Some(previous) if previous == certificate.as_ref() => Ok(()),
    Some(previous) if previous_certificate_expired(&previous) =>
      store.save(hostname, port, certificate),
    Some(_) =>
      anyhow::bail!("Gemini server certificate changed for {hostname}:{port}"),
    None => store.save(hostname, port, certificate),
  }
}

fn previous_certificate_expired(certificate: &[u8]) -> bool {
  let Ok(certificate) = x509_cert::Certificate::from_der(certificate) else {
    return false;
  };
  let expires_at = SystemTime::UNIX_EPOCH
    + certificate.tbs_certificate.validity.not_after.to_unix_duration();

  expires_at < SystemTime::now()
}

/// Return the Mozilla CA roots used by the default request functions.
///
/// Callers can add a certificate to this store before passing it to
/// `request_with_roots` to trust a selected self-signed Gemini capsule.
#[must_use]
pub fn default_root_certificates() -> RootCertStore {
  RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() }
}

pub fn client_config(roots: RootCertStore) -> Result<ClientConfig, Error> {
  Ok(
    client_config_builder()?
      .with_root_certificates(roots)
      .with_no_client_auth(),
  )
}

#[cfg(test)]
mod tests {
  use {
    super::{CertificateStore, TrustOnFirstUseVerifier, check_tofu},
    rustls::{
      CertificateError, Error,
      client::danger::ServerCertVerifier,
      pki_types::{CertificateDer, ServerName, UnixTime},
    },
    std::collections::HashMap,
  };

  #[derive(Default)]
  struct MemoryCertificates(HashMap<(String, u16), Vec<u8>>);

  impl CertificateStore for MemoryCertificates {
    fn load(
      &mut self,
      hostname: &str,
      port: u16,
    ) -> anyhow::Result<Option<Vec<u8>>> {
      Ok(self.0.get(&(hostname.to_owned(), port)).cloned())
    }

    fn save(
      &mut self,
      hostname: &str,
      port: u16,
      certificate: &[u8],
    ) -> anyhow::Result<()> {
      self.0.insert((hostname.to_owned(), port), certificate.to_vec());

      Ok(())
    }
  }

  fn verify_certificate(
    parameters: &rcgen::CertificateParams,
  ) -> Result<(), Error> {
    let key_pair = rcgen::KeyPair::generate().unwrap();
    let certificate = parameters.self_signed(&key_pair).unwrap();

    TrustOnFirstUseVerifier::new().verify_server_cert(
      certificate.der(),
      &[],
      &ServerName::try_from("localhost").unwrap(),
      &[],
      UnixTime::now(),
    )?;

    Ok(())
  }

  #[test]
  fn accepts_certificate_with_unrelated_common_name() {
    let mut parameters = rcgen::CertificateParams::default();

    parameters
      .distinguished_name
      .push(rcgen::DnType::CommonName, "elsewhere.test");

    verify_certificate(&parameters).unwrap();
  }

  #[test]
  fn rejects_malformed_certificate() {
    let error = TrustOnFirstUseVerifier::new()
      .verify_server_cert(
        &CertificateDer::from(vec![1, 2, 3]),
        &[],
        &ServerName::try_from("localhost").unwrap(),
        &[],
        UnixTime::now(),
      )
      .unwrap_err();

    assert_eq!(error, Error::InvalidCertificate(CertificateError::BadEncoding));
  }

  #[test]
  fn pins_are_scoped_to_hostname_and_port() {
    let certificate = CertificateDer::from(vec![1, 2, 3]);
    let other_certificate = CertificateDer::from(vec![4, 5, 6]);
    let mut store = MemoryCertificates::default();

    check_tofu(&mut store, "one.test", 1965, Some(&certificate)).unwrap();
    check_tofu(&mut store, "one.test", 1965, Some(&certificate)).unwrap();
    check_tofu(&mut store, "one.test", 1966, Some(&other_certificate)).unwrap();
    check_tofu(&mut store, "two.test", 1965, Some(&other_certificate)).unwrap();
    assert_eq!(store.0.len(), 3);
    assert!(
      check_tofu(&mut store, "one.test", 1965, Some(&other_certificate))
        .is_err()
    );
  }

  #[test]
  fn replaces_an_expired_certificate() {
    let mut parameters = rcgen::CertificateParams::default();

    parameters.distinguished_name.push(rcgen::DnType::CommonName, "localhost");

    parameters.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    parameters.not_before = rcgen::date_time_ymd(2018, 1, 1);
    parameters.not_after = rcgen::date_time_ymd(2019, 1, 1);

    let key_pair = rcgen::KeyPair::generate().unwrap();
    let expired_certificate =
      parameters.self_signed(&key_pair).unwrap().der().to_vec();
    let replacement = CertificateDer::from(vec![7, 8, 9]);
    let mut store = MemoryCertificates::default();

    store.0.insert(("localhost".to_owned(), 1965), expired_certificate);
    check_tofu(&mut store, "localhost", 1965, Some(&replacement)).unwrap();
    assert_eq!(
      store.0.get(&("localhost".to_owned(), 1965)),
      Some(&replacement.to_vec())
    );
  }
}
