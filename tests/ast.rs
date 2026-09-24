#![cfg(feature = "ast")]

#[cfg(test)]
mod test {
  use germ::ast::{Ast, Node};

  const EXAMPLE_GEMTEXT: &str = include_str!("../examples/example.gmi");

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

  #[test]
  fn gemtext_to_ast_then_ast_to_gemtext() {
    assert_eq!(Ast::from_string(EXAMPLE_GEMTEXT).to_gemtext(), EXAMPLE_GEMTEXT);
  }

  #[cfg(feature = "macros")]
  #[test]
  fn gemtext_to_ast_then_ast_to_gemtext_macro_expression() {
    assert_eq!(
      germ::gemini_to_ast!(EXAMPLE_GEMTEXT).to_gemtext(),
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
    let ast = Ast::from_string("# Hello, 世界!");

    assert_eq!(ast.inner().len(), 1);

    if let Node::Heading { level, text } = ast.inner().first().unwrap() {
      assert_eq!(level, &1);
      assert_eq!(text, "Hello, 世界!");
    } else {
      panic!("Expected heading node");
    }

    let ast = Ast::from_string("###");

    assert_eq!(ast.inner().len(), 1);

    if let Node::Heading { level, text } = ast.inner().first().unwrap() {
      assert_eq!(level, &3);
      assert_eq!(text, "");
    } else {
      panic!("Expected heading node");
    }

    let ast = Ast::from_string("########## Very Deep Heading");

    assert_eq!(ast.inner(), &vec![Node::Text(
      "########## Very Deep Heading".to_string()
    )]);
  }

  #[test]
  fn recognises_only_exact_line_markers() {
    let ast = Ast::from_string(
      "*star\n=text\n`tick\n#### too deep\n* item\n*  indented",
    );

    assert_eq!(ast.inner(), &vec![
      Node::Text("*star".to_string()),
      Node::Text("=text".to_string()),
      Node::Text("`tick".to_string()),
      Node::Text("#### too deep".to_string()),
      Node::List(vec!["item".to_string(), " indented".to_string()]),
    ]);
  }

  #[test]
  fn preserves_link_label_spacing() {
    let source = "=> /path label  with   spaces";
    let ast = Ast::from_string(source);

    assert_eq!(ast.inner(), &vec![Node::Link {
      to:   "/path".to_string(),
      text: Some("label  with   spaces".to_string()),
    }]);
    assert_eq!(ast.to_gemtext(), source);
  }

  #[test]
  fn preserves_unclosed_preformatted_content() {
    let source = "```alt\n* item\n=> /path\n# heading";
    let ast = Ast::from_string(source);

    assert_eq!(ast.inner(), &vec![Node::PreformattedText {
      alt_text: Some("alt".to_string()),
      text:     "* item\n=> /path\n# heading".to_string(),
    }]);
    assert_eq!(ast.to_gemtext(), source);

    let source_with_final_newline = "```alt\ncode\n";
    let ast = Ast::from_string(source_with_final_newline);

    assert_eq!(ast.inner(), &vec![Node::PreformattedText {
      alt_text: Some("alt".to_string()),
      text:     "code\n".to_string(),
    }]);
    assert_eq!(ast.to_gemtext(), source_with_final_newline);
  }

  #[test]
  fn preserves_canonical_line_round_trips() {
    for source in [
      "",
      "plain",
      "plain\n",
      "plain\n\n",
      "\n",
      "\n\n",
      "* item\n*  indented\nnext",
      "* item\n\n",
      "# heading\n###\n> quoted",
      "```\n```\n",
      "```\ncode\n```",
      "```\ncode\n```\n\n",
      "```\ncode\n",
      "```",
      "```\n",
      "```\n\n",
      "plain\r\n\r\n* item\r\n",
      "```\r\ncode\r\n```\r\n",
    ] {
      assert_eq!(Ast::from_string(source).to_gemtext(), source, "{source:?}");
    }
  }

  #[test]
  fn normalises_mixed_line_endings_and_optional_marker_spaces() {
    let source = "#  Heading\r\n=>  /path  label\n>  quote\r\n";
    let expected = "#  Heading\r\n=> /path label\r\n>  quote\r\n";

    assert_eq!(Ast::from_string(source).to_gemtext(), expected);
  }
}
