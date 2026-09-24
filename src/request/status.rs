use std::{fmt, fmt::Formatter};

/// A Gemini response status.
///
/// # Examples
///
/// ```rust
/// use germ::request::Status;
///
/// assert_eq!(Status::from(10), Status::Input);
/// assert_eq!(i32::from(Status::Input), 10);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Status {
  Input,
  SensitiveInput,
  #[default]
  Success,
  TemporaryRedirect,
  PermanentRedirect,
  TemporaryFailure,
  ServerUnavailable,
  CGIError,
  ProxyError,
  SlowDown,
  PermanentFailure,
  NotFound,
  Gone,
  ProxyRefused,
  BadRequest,
  ClientCertificateRequired,
  CertificateNotAuthorised,
  CertificateNotValid,
  /// A code without a named variant.
  Unknown(i32),
  /// A legacy status without a known code.
  Unsupported,
}

/// The response category indicated by the first status digit.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusCategory {
  Input,
  Success,
  Redirect,
  TemporaryFailure,
  PermanentFailure,
  ClientCertificateRequired,
}

impl Status {
  /// Returns the category for codes in the Gemini status range.
  #[must_use]
  pub fn category(self) -> Option<StatusCategory> {
    match i32::from(self) {
      10..=19 => Some(StatusCategory::Input),
      20..=29 => Some(StatusCategory::Success),
      30..=39 => Some(StatusCategory::Redirect),
      40..=49 => Some(StatusCategory::TemporaryFailure),
      50..=59 => Some(StatusCategory::PermanentFailure),
      60..=69 => Some(StatusCategory::ClientCertificateRequired),
      _ => None,
    }
  }
}

impl From<Status> for i32 {
  fn from(status: Status) -> Self {
    match status {
      Status::Input => 10,
      Status::SensitiveInput => 11,
      Status::Success => 20,
      Status::TemporaryRedirect => 30,
      Status::PermanentRedirect => 31,
      Status::TemporaryFailure => 40,
      Status::ServerUnavailable => 41,
      Status::CGIError => 42,
      Status::ProxyError => 43,
      Status::SlowDown => 44,
      Status::PermanentFailure => 50,
      Status::NotFound => 51,
      Status::Gone => 52,
      Status::ProxyRefused => 53,
      Status::BadRequest => 59,
      Status::ClientCertificateRequired => 60,
      Status::CertificateNotAuthorised => 61,
      Status::CertificateNotValid => 62,
      Status::Unknown(code) => code,
      Status::Unsupported => 0,
    }
  }
}

impl From<i32> for Status {
  fn from(code: i32) -> Self {
    match code {
      10 => Self::Input,
      11 => Self::SensitiveInput,
      20 => Self::Success,
      30 => Self::TemporaryRedirect,
      31 => Self::PermanentRedirect,
      40 => Self::TemporaryFailure,
      41 => Self::ServerUnavailable,
      42 => Self::CGIError,
      43 => Self::ProxyError,
      44 => Self::SlowDown,
      50 => Self::PermanentFailure,
      51 => Self::NotFound,
      52 => Self::Gone,
      53 => Self::ProxyRefused,
      59 => Self::BadRequest,
      60 => Self::ClientCertificateRequired,
      61 => Self::CertificateNotAuthorised,
      62 => Self::CertificateNotValid,
      _ => Self::Unknown(code),
    }
  }
}

impl fmt::Display for Status {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
