use {
  rustls::{
    Certificate, CertificateError, ClientConfig, Error, OwnedTrustAnchor,
    RootCertStore, ServerName,
    client::{ServerCertVerified, ServerCertVerifier},
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
struct TrustOnFirstUseVerifier;

impl ServerCertVerifier for TrustOnFirstUseVerifier {
  fn verify_server_cert(
    &self,
    end_entity: &Certificate,
    _intermediates: &[Certificate],
    _server_name: &ServerName,
    _scts: &mut dyn Iterator<Item = &[u8]>,
    _ocsp_response: &[u8],
    _now: SystemTime,
  ) -> Result<ServerCertVerified, Error> {
    x509_cert::Certificate::from_der(&end_entity.0)
      .map_err(|_| Error::InvalidCertificate(CertificateError::BadEncoding))?;

    // TOFU pins the exact certificate before sending the URL. Its expiry
    // controls pin replacement; rustls verifies the TLS handshake signature.
    Ok(ServerCertVerified::assertion())
  }
}

pub fn tofu_client_config() -> ClientConfig {
  ClientConfig::builder()
    .with_safe_defaults()
    .with_custom_certificate_verifier(Arc::new(TrustOnFirstUseVerifier))
    .with_no_client_auth()
}

pub fn check_tofu(
  store: &mut dyn CertificateStore,
  hostname: &str,
  port: u16,
  certificate: Option<&Certificate>,
) -> anyhow::Result<()> {
  let certificate = certificate.ok_or_else(|| {
    anyhow::anyhow!("Gemini server did not provide a certificate")
  })?;

  match store.load(hostname, port)? {
    Some(previous) if previous == certificate.0 => Ok(()),
    Some(previous) if previous_certificate_expired(&previous) =>
      store.save(hostname, port, &certificate.0),
    Some(_) =>
      anyhow::bail!("Gemini server certificate changed for {hostname}:{port}"),
    None => store.save(hostname, port, &certificate.0),
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
  let mut roots = RootCertStore::empty();

  roots.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(
    |anchor| {
      OwnedTrustAnchor::from_subject_spki_name_constraints(
        anchor.subject.as_ref().to_vec(),
        anchor.subject_public_key_info.as_ref().to_vec(),
        anchor
          .name_constraints
          .as_ref()
          .map(|constraints| constraints.as_ref().to_vec()),
      )
    },
  ));

  roots
}

pub fn client_config(roots: RootCertStore) -> ClientConfig {
  ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(roots)
    .with_no_client_auth()
}

#[cfg(test)]
mod tests {
  use {
    super::{CertificateStore, TrustOnFirstUseVerifier, check_tofu},
    rustls::{
      Certificate, CertificateError, Error, ServerName,
      client::ServerCertVerifier,
    },
    std::{collections::HashMap, time::SystemTime},
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
    parameters: rcgen::CertificateParams,
  ) -> Result<(), Error> {
    let certificate = rcgen::Certificate::from_params(parameters).unwrap();
    let certificate = Certificate(certificate.serialize_der().unwrap());

    TrustOnFirstUseVerifier.verify_server_cert(
      &certificate,
      &[],
      &ServerName::try_from("localhost").unwrap(),
      &mut std::iter::empty(),
      &[],
      SystemTime::now(),
    )?;

    Ok(())
  }

  #[test]
  fn accepts_certificate_with_unrelated_common_name() {
    let mut parameters = rcgen::CertificateParams::default();

    parameters
      .distinguished_name
      .push(rcgen::DnType::CommonName, "elsewhere.test");

    verify_certificate(parameters).unwrap();
  }

  #[test]
  fn rejects_malformed_certificate() {
    let error = TrustOnFirstUseVerifier
      .verify_server_cert(
        &Certificate(vec![1, 2, 3]),
        &[],
        &ServerName::try_from("localhost").unwrap(),
        &mut std::iter::empty(),
        &[],
        SystemTime::now(),
      )
      .unwrap_err();

    assert_eq!(error, Error::InvalidCertificate(CertificateError::BadEncoding));
  }

  #[test]
  fn pins_are_scoped_to_hostname_and_port() {
    let certificate = Certificate(vec![1, 2, 3]);
    let other_certificate = Certificate(vec![4, 5, 6]);
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

    let expired_certificate = rcgen::Certificate::from_params(parameters)
      .unwrap()
      .serialize_der()
      .unwrap();
    let replacement = Certificate(vec![7, 8, 9]);
    let mut store = MemoryCertificates::default();

    store.0.insert(("localhost".to_owned(), 1965), expired_certificate);
    check_tofu(&mut store, "localhost", 1965, Some(&replacement)).unwrap();
    assert_eq!(
      store.0.get(&("localhost".to_owned(), 1965)),
      Some(&replacement.0)
    );
  }
}
