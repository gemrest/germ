#![cfg(any(feature = "request", feature = "blocking"))]

#[cfg(test)]
mod test {
  use germ::request::Status;

  #[test]
  fn status_from_i32() {
    assert_eq!(Status::from(10), Status::Input);
  }

  #[test]
  fn i32_from_status() {
    assert_eq!(i32::from(Status::Input), 10);
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
