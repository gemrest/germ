#[cfg(test)]
mod test {
  #[test]
  fn node_to_gemtext() {
    assert_eq!(
      germ::ast::Node::Link {
        to:   "/faq".to_string(),
        text: Some("FAQ".to_string()),
      }
      .to_gemtext(),
      "=> /faq FAQ",
    );
  }
}
