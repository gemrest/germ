use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};

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
