#![cfg(feature = "convert")]

use germ::{
  ast::{Ast, Node},
  convert::{
    self, Target,
    html::{self, Renderer},
  },
};

#[test]
fn default_hooks_preserve_existing_output() {
  let source = Ast::from_nodes(vec![
    Node::Text("<&>\"'".to_owned()),
    Node::Link {
      to:   "/next?one=1&two=\"2\"".to_owned(),
      text: Some("Next <page>".to_owned()),
    },
    Node::Link { to: "/unlabelled".to_owned(), text: None },
    Node::Link {
      to:   "javascript:alert(1)".to_owned(),
      text: Some("Unsafe <link>".to_owned()),
    },
    Node::Heading { level: 1, text: "First".to_owned() },
    Node::Heading { level: 2, text: "Second".to_owned() },
    Node::Heading { level: 3, text: "Third".to_owned() },
    Node::Heading { level: 4, text: "Other".to_owned() },
    Node::List(vec!["One".to_owned(), "<&>".to_owned()]),
    Node::List(vec![]),
    Node::Blockquote("Quote <&>".to_owned()),
    Node::PreformattedText {
      alt_text: Some("<ignored>".to_owned()),
      text:     "<code>\n".to_owned(),
    },
    Node::Whitespace,
  ]);
  let expected = concat!(
    "<p>&lt;&amp;&gt;&quot;&#39;</p>",
    "<a href=\"/next?one=1&amp;two=&quot;2&quot;\">Next &lt;page&gt;</a><br>",
    "<a href=\"/unlabelled\">/unlabelled</a><br>",
    "Unsafe &lt;link&gt;<br>",
    "<h1>First</h1><h2>Second</h2><h3>Third</h3><p>Other</p>",
    "<ul><li>One</li><li>&lt;&amp;&gt;</li></ul><ul></ul>",
    "<blockquote>Quote &lt;&amp;&gt;</blockquote><pre>&lt;code&gt;\n</pre>",
  );

  assert_eq!(convert::from_ast(&source, &Target::HTML), expected);
  assert_eq!(html::from_ast(&source, &mut ()), expected);
  assert_eq!(
    html::from_string("<script>\n\n```\ncode\n```", &mut ()),
    convert::from_string("<script>\n\n```\ncode\n```", &Target::HTML),
  );
}

#[derive(Default)]
struct InlineFormatter {
  text_inputs: Vec<String>,
}

impl Renderer for InlineFormatter {
  fn format_text(&mut self, text: &str, html: &mut String) {
    self.text_inputs.push(text.to_owned());
    html.push_str("<em>");
    ().format_text(text, html);
    html.push_str("</em>");
  }
}

#[test]
fn customizes_text_across_node_types() {
  let source = Ast::from_nodes(vec![
    Node::Text("Text".to_owned()),
    Node::Heading { level: 1, text: "First".to_owned() },
    Node::Heading { level: 2, text: "Second".to_owned() },
    Node::Heading { level: 3, text: "Third".to_owned() },
    Node::Heading { level: 4, text: "Other".to_owned() },
    Node::List(vec!["One".to_owned(), "Two".to_owned()]),
    Node::Blockquote("Quote".to_owned()),
    Node::Link { to: "/next".to_owned(), text: Some("Next".to_owned()) },
    Node::Link { to: "/unlabelled".to_owned(), text: None },
    Node::Link {
      to:   "javascript:alert(1)".to_owned(),
      text: Some("Unsafe".to_owned()),
    },
    Node::PreformattedText {
      alt_text: Some("Not formatted".to_owned()),
      text:     "<raw>\n".to_owned(),
    },
    Node::Whitespace,
  ]);
  let mut formatter = InlineFormatter::default();
  let result = html::from_ast(&source, &mut formatter);

  assert_eq!(
    result,
    concat!(
      "<p><em>Text</em></p><h1><em>First</em></h1>",
      "<h2><em>Second</em></h2><h3><em>Third</em></h3><p><em>Other</em></p>",
      "<ul><li><em>One</em></li><li><em>Two</em></li></ul>",
      "<blockquote><em>Quote</em></blockquote>",
      "<a href=\"/next\"><em>Next</em></a><br>",
      "<a href=\"/unlabelled\"><em>/unlabelled</em></a><br>",
      "<em>Unsafe</em><br><pre>&lt;raw&gt;\n</pre>",
    ),
  );
  assert_eq!(formatter.text_inputs, [
    "Text",
    "First",
    "Second",
    "Third",
    "Other",
    "One",
    "Two",
    "Quote",
    "Next",
    "/unlabelled",
    "Unsafe",
  ],);
}

#[test]
fn text_hooks_preserve_standard_link_safety_and_attribute_escaping() {
  let mut formatter = InlineFormatter::default();

  assert_eq!(
    html::from_string(
      "=> /path?x=\"<&>' <script>\n=> javascript:alert(1) <unsafe>\n=> \
       data:text/html,<script>\nplain <script>",
      &mut formatter,
    ),
    concat!(
      "<a href=\"/path?x=&quot;&lt;&amp;&gt;&#39;\"><em>&lt;script&gt;</em>",
      "</a><br><em>&lt;unsafe&gt;</em><br>",
      "<em>data:text/html,&lt;script&gt;</em><br>",
      "<p><em>plain &lt;script&gt;</em></p>",
    ),
  );
}

#[derive(Default)]
struct LinkGroups {
  open:      bool,
  condensed: bool,
  observed:  Vec<Node>,
  finishes:  usize,
}

impl LinkGroups {
  fn close_links(&mut self, html: &mut String) {
    if self.open {
      html.push_str("</p>");

      self.open = false;
    }
  }
}

impl Renderer for LinkGroups {
  fn render_node(&mut self, node: &Node, html: &mut String) -> bool {
    self.observed.push(node.clone());

    if !matches!(node, Node::Link { .. }) {
      self.close_links(html);
    }

    if let Node::Heading { text, .. } = node {
      self.condensed = text == "Compact";
    }

    let Node::Link { to, text } = node else {
      return false;
    };
    let label = text.as_deref().unwrap_or(to);

    if self.open {
      html.push_str(if self.condensed { " | " } else { "<br />" });
    } else {
      html.push_str("<p>");
    }

    html.push_str("<a href=\"");
    ().format_text(to, html);
    html.push_str("\">");
    ().format_text(label, html);
    html.push_str("</a>");

    self.open = true;

    true
  }

  fn finish(&mut self, html: &mut String) {
    self.close_links(html);

    self.finishes += 1;
  }
}

#[test]
fn groups_links_across_whitespace_and_heading_boundaries_and_finalizes() {
  let source = Ast::from_string(concat!(
    "# Compact\n=> /one One\n=> /two Two\n\n=> /three Three\n",
    "# Full\n=> /four Four\n=> /five Five\ntext\n=> /six Six",
  ));
  let mut renderer = LinkGroups::default();
  let result = html::from_ast(&source, &mut renderer);

  assert_eq!(
    result,
    concat!(
      "<h1>Compact</h1><p><a href=\"/one\">One</a> | ",
      "<a href=\"/two\">Two</a></p><p><a href=\"/three\">Three</a></p>",
      "<h1>Full</h1><p><a href=\"/four\">Four</a><br />",
      "<a href=\"/five\">Five</a></p><p>text</p>",
      "<p><a href=\"/six\">Six</a></p>",
    ),
  );
  assert_eq!(&renderer.observed, source.inner());
  assert!(!renderer.open);
  assert_eq!(renderer.finishes, 1);
}

#[test]
fn finalizes_empty_documents_once() {
  struct Footer {
    finishes: usize,
  }

  impl Renderer for Footer {
    fn finish(&mut self, html: &mut String) {
      html.push_str("<!-- finished -->");

      self.finishes += 1;
    }
  }

  let mut renderer = Footer { finishes: 0 };

  assert_eq!(html::from_string("", &mut renderer), "<!-- finished -->");
  assert_eq!(renderer.finishes, 1);
}

#[test]
fn custom_nodes_emit_html_only_when_explicitly_handled() {
  struct CustomText;

  impl Renderer for CustomText {
    fn render_node(&mut self, node: &Node, html: &mut String) -> bool {
      if matches!(node, Node::Text(_)) {
        html.push_str("<aside>Caller HTML</aside>");

        return true;
      }

      false
    }

    fn format_text(&mut self, _text: &str, _html: &mut String) {
      panic!("A handled node must bypass standard formatting.");
    }
  }

  assert_eq!(
    html::from_string("<script>\n\n```\n<script>\n```", &mut CustomText),
    "<aside>Caller HTML</aside><pre>&lt;script&gt;\n</pre>",
  );
  assert_eq!(
    convert::from_string("<script>", &Target::HTML),
    "<p>&lt;script&gt;</p>",
  );
}

#[test]
fn trims_one_preformatted_newline_before_escaping_and_separates_list_items() {
  struct NewlineFormatting;

  impl Renderer for NewlineFormatting {
    fn format_preformatted(&mut self, text: &str, html: &mut String) {
      ().format_preformatted(text.strip_suffix('\n').unwrap_or(text), html);
    }

    fn list_item_separator(&self) -> &'static str { "\n" }
  }

  let source = Ast::from_nodes(vec![
    Node::List(vec!["One".to_owned(), "Two".to_owned()]),
    Node::PreformattedText {
      alt_text: None,
      text:     "<&>\"'\n\n".to_owned(),
    },
    Node::PreformattedText {
      alt_text: None,
      text:     "no newline".to_owned(),
    },
    Node::PreformattedText { alt_text: None, text: "".to_owned() },
    Node::PreformattedText {
      alt_text: None,
      text:     "<code>\r\n".to_owned(),
    },
  ]);

  assert_eq!(
    html::from_ast(&source, &mut NewlineFormatting),
    concat!(
      "<ul><li>One</li>\n<li>Two</li></ul>",
      "<pre>&lt;&amp;&gt;&quot;&#39;\n</pre><pre>no newline</pre><pre></pre>",
      "<pre>&lt;code&gt;\r</pre>",
    ),
  );
}
