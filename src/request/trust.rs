use {
  rustls::{
    Certificate, CertificateError, ClientConfig, Error, OwnedTrustAnchor,
    RootCertStore, ServerName,
    client::{ServerCertVerified, ServerCertVerifier, WebPkiVerifier},
  },
  std::{sync::Arc, time::SystemTime},
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
    intermediates: &[Certificate],
    server_name: &ServerName,
    scts: &mut dyn Iterator<Item = &[u8]>,
    ocsp_response: &[u8],
    now: SystemTime,
  ) -> Result<ServerCertVerified, Error> {
    let mut roots = RootCertStore::empty();

    roots.add(end_entity)?;

    // The presented leaf is the temporary trust anchor; rustls still checks
    // its validity, server name, and handshake signature.
    WebPkiVerifier::new(roots, None).verify_server_cert(
      end_entity,
      intermediates,
      server_name,
      scts,
      ocsp_response,
      now,
    )
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
    Some(previous) if previous_certificate_expired(&previous, hostname) =>
      store.save(hostname, port, &certificate.0),
    Some(_) =>
      anyhow::bail!("Gemini server certificate changed for {hostname}:{port}"),
    None => store.save(hostname, port, &certificate.0),
  }
}

fn previous_certificate_expired(certificate: &[u8], hostname: &str) -> bool {
  let certificate = Certificate(certificate.to_vec());
  let Ok(server_name) = ServerName::try_from(hostname) else {
    return false;
  };
  let mut roots = RootCertStore::empty();

  if roots.add(&certificate).is_err() {
    return false;
  }

  // Revalidate the stored certificate to detect expiry without parsing its
  // validity dates separately.
  let result = WebPkiVerifier::new(roots, None).verify_server_cert(
    &certificate,
    &[],
    &server_name,
    &mut std::iter::empty(),
    &[],
    SystemTime::now(),
  );

  matches!(result, Err(Error::InvalidCertificate(CertificateError::Expired)))
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
    super::{CertificateStore, check_tofu},
    rustls::Certificate,
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
    let mut parameters =
      rcgen::CertificateParams::new(vec!["localhost".to_owned()]);

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
