// This file is part of Germ <https://github.com/gemrest/germ>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

/// Simple Gemini status reporting
///
/// # Examples
///
/// ```rust
/// use germ::request::Status;
///
/// assert_eq!(Status::from(10), Status::Input);
/// assert_eq!(i32::from(Status::Input), 10);
/// ```
#[derive(Debug, PartialEq)]
pub enum Status {
  Input,
  SensitiveInput,
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
  Unsupported,
}
impl Default for Status {
  fn default() -> Self { Self::Success }
}
impl From<Status> for i32 {
  fn from(n: Status) -> Self {
    match n {
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
      Status::Unsupported => 0,
    }
  }
}
impl From<i32> for Status {
  fn from(n: i32) -> Self {
    match n {
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
      _ => Self::Unsupported,
    }
  }
}
