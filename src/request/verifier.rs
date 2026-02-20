use {
  rustls::{
    Certificate,
    client::{self, ServerCertVerified},
  },
  std::time::SystemTime,
};

#[allow(clippy::module_name_repetitions)]
pub struct GermVerifier;

impl GermVerifier {
  pub const fn new() -> Self { Self {} }
}

impl client::ServerCertVerifier for GermVerifier {
  fn verify_server_cert(
    &self,
    _end_entity: &Certificate,
    _intermediates: &[Certificate],
    _server_name: &client::ServerName,
    _scts: &mut dyn Iterator<Item = &[u8]>,
    _ocsp_response: &[u8],
    _now: SystemTime,
  ) -> Result<ServerCertVerified, rustls::Error> {
    Ok(ServerCertVerified::assertion())
  }
}
