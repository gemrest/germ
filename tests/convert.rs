#![cfg(feature = "convert")]

#[cfg(test)]
mod test {
  use germ::{
    ast::{Ast, Node},
    convert::{Target, from_ast, from_string},
  };
  #[cfg(feature = "macros")] use germ::{gemini_to_html, gemini_to_md};

  #[test]
  fn convert_from_string_to_html_single_line() {
    assert_eq!(from_string("hi", &Target::HTML), "<p>hi</p>",);
  }

  #[test]
  fn convert_from_string_to_html_multi_line() {
    assert_eq!(from_string("hi\n# hi", &Target::HTML), "<p>hi</p><h1>hi</h1>",);
  }

  #[test]
  fn html_escapes_content_in_every_node() {
    let hostile = "<&>\"'".to_owned();
    let ast = Ast::from_nodes(vec![
      Node::Text(hostile.clone()),
      Node::Heading { level: 1, text: hostile.clone() },
      Node::List(vec![hostile.clone()]),
      Node::Blockquote(hostile.clone()),
      Node::PreformattedText { alt_text: None, text: hostile },
    ]);

    assert_eq!(
      from_ast(&ast, &Target::HTML),
      "<p>&lt;&amp;&gt;&quot;&#39;</p><h1>&lt;&amp;&gt;&quot;&#39;</\
       h1><ul><li>&lt;&amp;&gt;&quot;&#39;</li></ul><blockquote>&lt;&amp;&gt;&\
       quot;&#39;</blockquote><pre>&lt;&amp;&gt;&quot;&#39;</pre>",
    );
  }

  #[test]
  fn html_escapes_link_targets_and_labels() {
    let ast = Ast::from_nodes(vec![Node::Link {
      to:   "/path?x=\"<&>".to_owned(),
      text: Some("<script>alert('x')</script>".to_owned()),
    }]);

    assert_eq!(
      from_ast(&ast, &Target::HTML),
      "<a href=\"/path?x=&quot;&lt;&amp;&gt;\">&lt;script&gt;alert(&#39;x&#39;\
       )&lt;/script&gt;</a><br>",
    );
  }

  #[test]
  fn html_renders_unsafe_links_as_text() {
    for target in [
      "javascript:alert(1)",
      "JaVaScRiPt:alert(1)",
      "data:text/html,<script>alert(1)</script>",
      "java\nscript:alert(1)",
      "vbscript:msgbox(1)",
      "sftp://example.com/file",
    ] {
      let ast = Ast::from_nodes(vec![Node::Link {
        to:   target.to_owned(),
        text: Some("safe label".to_owned()),
      }]);

      assert_eq!(from_ast(&ast, &Target::HTML), "safe label<br>");
    }
  }

  #[test]
  fn html_escapes_raw_gemtext() {
    assert_eq!(
      from_string(
        "<script>alert(1)</script>\n=> javascript:alert(2) click",
        &Target::HTML
      ),
      "<p>&lt;script&gt;alert(1)&lt;/script&gt;</p>click<br>",
    );
  }

  #[test]
  fn html_keeps_safe_relative_and_absolute_links() {
    for target in [
      "/local",
      "relative",
      "#anchor",
      "gemini://example.com",
      "https://example.com",
    ] {
      let ast = Ast::from_nodes(vec![Node::Link {
        to:   target.to_owned(),
        text: None,
      }]);

      assert_eq!(
        from_ast(&ast, &Target::HTML),
        format!("<a href=\"{target}\">{target}</a><br>"),
      );
    }
  }

  #[cfg(feature = "macros")]
  #[test]
  fn convert_from_string_to_html_single_link_macro_expression() {
    assert_eq!(
      gemini_to_html!("=> /to hello !"),
      "<a href=\"/to\">hello !</a><br>",
    );
  }

  #[test]
  fn convert_from_string_to_markdown_single_line() {
    assert_eq!(from_string("hi", &Target::Markdown), "hi\n",);
  }

  #[test]
  fn convert_from_string_to_markdown_multi_line() {
    assert_eq!(from_string("hi\n# hi", &Target::Markdown), "hi\n# hi\n",);
  }

  #[test]
  fn convert_from_string_to_markdown_single_link() {
    assert_eq!(
      from_string("=> /to hello !", &Target::Markdown),
      "[hello !](/to)\n",
    );
  }

  #[test]
  fn renderers_keep_unclosed_preformatted_content() {
    assert_eq!(
      from_string("```alt\n* item\n# heading", &Target::HTML),
      "<pre>* item\n# heading</pre>"
    );
    assert_eq!(
      from_string("```alt\n* item\n# heading", &Target::Markdown),
      "```alt\n* item\n# heading\n```\n"
    );
    assert_eq!(
      from_string("```alt\ncode\n", &Target::HTML),
      "<pre>code\n</pre>"
    );
  }

  #[test]
  fn markdown_separates_a_fenced_block_from_following_text() {
    assert_eq!(
      from_string("```\ncode\n```\nafter", &Target::Markdown),
      "```\ncode\n```\nafter\n"
    );
  }

  #[cfg(feature = "macros")]
  #[test]
  fn convert_from_string_to_markdown_single_macro_expression() {
    assert_eq!(gemini_to_md!("=> /to hello !"), "[hello !](/to)\n",);
  }
}
