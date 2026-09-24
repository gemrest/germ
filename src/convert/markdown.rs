use {crate::ast::Node, std::fmt::Write};

pub fn convert(source: &[Node]) -> String {
  let mut markdown = String::new();

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
        markdown.push_str("```");

        if let Some(alt_text) = alt_text {
          markdown.push_str(alt_text);
        }

        markdown.push('\n');
        markdown.push_str(text);

        if !text.is_empty() && !text.ends_with('\n') {
          markdown.push('\n');
        }

        markdown.push_str("```\n");
      }

      Node::Whitespace => markdown.push('\n'),
    }
  }

  markdown
}
