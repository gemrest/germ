#[cfg(test)]
mod test {
  use germ::meta::Meta;

  #[test]
  fn construct_meta_with_mime() {
    let mut meta = Meta::new();

    *meta.mime_mut() = "text/gemini".to_string();

    assert_eq!(meta.to_string(), "text/gemini");
  }

  #[test]
  fn construct_meta_with_mime_and_parameters() {
    let mut meta = Meta::new();
    let mut parameters = std::collections::HashMap::new();

    parameters.insert("hi".to_string(), "2".to_string());
    parameters.insert("hi2".to_string(), "string=2".to_string());

    *meta.mime_mut() = "text/gemini".to_string();
    *meta.parameters_mut() = parameters;

    assert_eq!(meta.to_string(), "text/gemini; hi=2; hi2=string=2");
  }

  #[test]
  fn meta_to_string_without_parameters() {
    let original_string = "text/gemini";

    assert_eq!(Meta::from_string(original_string).to_string(), original_string);
  }

  #[test]
  fn meta_to_string_with_parameters() {
    let original_string = "text/gemini; hi=2; hi2=string=2";

    assert_eq!(Meta::from_string(original_string).to_string(), original_string);
  }

  #[test]
  fn meta_to_mime_without_parameters() {
    let meta = Meta::from_string("text/gemini");

    assert_eq!(meta.mime(), "text/gemini");
    assert_eq!(meta.parameters().len(), 0);
  }

  #[test]
  fn meta_to_map_mime() {
    assert_eq!(
      Meta::from_string("text/gemini; hi=2; hi2=string=2").mime(),
      "text/gemini",
    );
  }

  #[test]
  fn meta_to_map_with_parameters() {
    assert_eq!(
      Meta::from_string("text/gemini; hi=2; hi2=string=2")
        .parameters()
        .get("hi2"),
      Some(&"string=2".to_string()),
    );
  }

  #[test]
  fn meta_to_map_length() {
    assert_eq!(
      Meta::from_string("text/gemini; hi=2; hi2=string=2").parameters().len(),
      2,
    );
  }

  #[test]
  fn meta_handles_malformed_parameters() {
    let meta =
      Meta::from_string("text/gemini; valid=value; malformed; another=test");

    assert_eq!(meta.mime(), "text/gemini");
    assert_eq!(meta.parameters().len(), 2);
    assert_eq!(meta.parameters().get("valid"), Some(&"value".to_string()));
    assert_eq!(meta.parameters().get("another"), Some(&"test".to_string()));
    assert_eq!(meta.parameters().get("malformed"), None);
  }
}
