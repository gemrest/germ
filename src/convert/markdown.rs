use {super::safe_link_target, crate::ast::Node, std::fmt::Write};

fn push_escaped_label(markdown: &mut String, label: &str) {
  for character in label.chars() {
    if character.is_control() {
      let _ = write!(markdown, "&#{};", u32::from(character));

      continue;
    }

    if matches!(
      character,
      '\\' | '`' | '*' | '_' | '[' | ']' | '<' | '>' | '&' | '|' | '~'
    ) {
      markdown.push('\\');
    }

    markdown.push(character);
  }
}

fn push_escaped_destination(markdown: &mut String, target: &str) {
  for character in target.chars() {
    match character {
      '(' | ')' => {
        markdown.push('\\');
        markdown.push(character);
      }
      '<' => markdown.push_str("%3C"),
      '>' => markdown.push_str("%3E"),
      '"' => markdown.push_str("%22"),
      '&' => markdown.push_str("&amp;"),
      _ => markdown.push(character),
    }
  }
}

pub fn convert(source: &[Node]) -> String {
  let mut markdown = String::new();

  for node in source {
    match node {
      Node::Text(text) => {
        let _ = writeln!(&mut markdown, "{text}");
      }

      Node::Link { to, text } => {
        let label = text.as_deref().unwrap_or(to);

        if safe_link_target(to) {
          markdown.push('[');
          push_escaped_label(&mut markdown, label);
          markdown.push_str("](");
          push_escaped_destination(&mut markdown, to);
          markdown.push_str(")\n");
        } else {
          push_escaped_label(&mut markdown, label);
          markdown.push('\n');
        }
      }

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
