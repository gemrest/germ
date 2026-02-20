use {crate::ast::Node, std::fmt::Write};

pub fn convert(source: &[Node]) -> String {
  let mut markdown = String::new();

  // Since we have an AST tree of the Gemtext, it is very easy to convert from
  // this AST tree to an alternative markup format.
  for node in source {
    match node {
      Node::Text(text) => {
        let _ = writeln!(&mut markdown, "{text}");
      }
      Node::Link { to, text } => markdown.push_str(&text.clone().map_or_else(
        || format!("<{to}>\n"),
        |text| format!("[{text}]({to})\n"),
      )),
      Node::Heading { level, text } => {
        let _ = writeln!(
          &mut markdown,
          "{} {}",
          match level {
            1 => "#",
            2 => "##",
            3 => "###",
            _ => "",
          },
          text
        );
      }
      Node::List(items) =>
        for item in items {
          let _ = writeln!(&mut markdown, "- {item}");
        },
      Node::Blockquote(text) => {
        let _ = writeln!(&mut markdown, "> {text}");
      }
      Node::PreformattedText { alt_text, text } => {
        let _ = writeln!(
          &mut markdown,
          "```{}\n{}```",
          alt_text.clone().unwrap_or_default(),
          text
        );
      }
      Node::Whitespace => markdown.push('\n'),
    }
  }

  markdown
}
