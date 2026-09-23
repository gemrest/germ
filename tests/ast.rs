#![cfg(feature = "ast")]

#[cfg(test)]
mod test {
  #[cfg(feature = "example-gemtext")] use germ::EXAMPLE_GEMTEXT;
  use germ::ast::{Ast, Node};

  #[test]
  fn build_multi_line_list_with_text() {
    assert_eq!(*Ast::from_string("* item1\n* 2\nhi text").inner(), vec![
      Node::List(vec!["item1".to_string(), "2".to_string()]),
      Node::Text("hi text".to_string()),
    ],);
  }

  #[test]
  fn build_multi_line_vec() {
    assert_eq!(*Ast::from_string("=> /test hi\nhi there\n> hi").inner(), vec![
      Node::Link { to: "/test".to_string(), text: Some("hi".to_string()) },
      Node::Text("hi there".to_string()),
      Node::Blockquote("hi".to_string()),
    ],);
  }

  #[test]
  fn build_single_0th_from_vec() {
    assert_eq!(Ast::from_string("=> /test hi").inner(), &vec![Node::Link {
      to:   "/test".to_string(),
      text: Some("hi".to_string()),
    }],);
  }

  #[test]
  fn build_single_element() {
    assert_eq!(
      Ast::from_string("=> /test hi").inner().first().unwrap(),
      &Node::Link { to: "/test".to_string(), text: Some("hi".to_string()) },
    );
  }

  #[cfg(feature = "example-gemtext")]
  #[test]
  fn gemtext_to_ast_then_ast_to_gemtext() {
    assert_eq!(
      Ast::from_string(EXAMPLE_GEMTEXT).to_gemtext(),
      // `to_gemtext` appends a newline to all responses, so let's make sure we
      // account for that.
      EXAMPLE_GEMTEXT
    );
  }

  #[cfg(all(feature = "example-gemtext", feature = "macros"))]
  #[test]
  fn gemtext_to_ast_then_ast_to_gemtext_macro_expression() {
    assert_eq!(
      germ::gemini_to_ast!(EXAMPLE_GEMTEXT).to_gemtext(),
      // `to_gemtext` appends a newline to all responses, so let's make sure we
      // account for that.
      EXAMPLE_GEMTEXT
    );
  }

  #[test]
  fn gemtext_to_ast_then_node_to_ast_to_gemtext() {
    assert_eq!(
      Ast::from_nodes(Ast::from_string("=> / Home").inner().to_vec())
        .to_gemtext(),
      "=> / Home"
    );
  }

  #[test]
  fn build_malformed_link_without_url() {
    let ast = Ast::from_string("=>");

    assert_eq!(ast.inner().len(), 1);

    if let Node::Link { to, text } = ast.inner().first().unwrap() {
      assert_eq!(to, "");
      assert_eq!(text, &None);
    } else {
      panic!("Expected link node");
    }
  }

  #[test]
  fn build_heading_with_unicode_and_edge_cases() {
    // Unicode characters
    let ast = Ast::from_string("# Hello, 世界!");

    assert_eq!(ast.inner().len(), 1);

    if let Node::Heading { level, text } = ast.inner().first().unwrap() {
      assert_eq!(level, &1);
      assert_eq!(text, "Hello, 世界!");
    } else {
      panic!("Expected heading node");
    }

    // Only hashes
    let ast = Ast::from_string("###");

    assert_eq!(ast.inner().len(), 1);

    if let Node::Heading { level, text } = ast.inner().first().unwrap() {
      assert_eq!(level, &3);
      assert_eq!(text, "");
    } else {
      panic!("Expected heading node");
    }

    // Many hashes
    let ast = Ast::from_string("########## Very Deep Heading");

    assert_eq!(ast.inner().len(), 1);

    if let Node::Heading { level, text } = ast.inner().first().unwrap() {
      assert_eq!(level, &10);
      assert_eq!(text, "Very Deep Heading");
    } else {
      panic!("Expected heading node");
    }
  }
}
