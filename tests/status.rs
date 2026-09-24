#![cfg(any(feature = "request", feature = "blocking"))]

#[cfg(test)]
mod test {
  use germ::request::{Status, StatusCategory};

  #[test]
  fn status_from_i32() {
    assert_eq!(Status::from(10), Status::Input);
  }

  #[test]
  fn i32_from_status() {
    assert_eq!(i32::from(Status::Input), 10);
  }

  #[test]
  fn unknown_codes_keep_their_value_and_category() {
    for (code, category) in [
      (19, StatusCategory::Input),
      (29, StatusCategory::Success),
      (39, StatusCategory::Redirect),
      (49, StatusCategory::TemporaryFailure),
      (58, StatusCategory::PermanentFailure),
      (69, StatusCategory::ClientCertificateRequired),
    ] {
      let status = Status::from(code);

      assert_eq!(status, Status::Unknown(code));
      assert_eq!(i32::from(status), code);
      assert_eq!(status.category(), Some(category));
    }

    assert_eq!(Status::from(79).category(), None);
    assert_eq!(i32::from(Status::from(79)), 79);
    assert_eq!(Status::Unsupported.category(), None);
  }

  #[cfg(feature = "blocking")]
  #[test]
  fn invalid_url_handling() {
    use url::Url;

    let invalid_url = Url::parse("gemini://").unwrap();
    let result = germ::request::blocking::request(&invalid_url);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("missing domain"));
  }
}
