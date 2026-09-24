use {super::safe_link_target, crate::ast::Node, std::fmt::Write};

fn push_escaped_text(markdown: &mut String, text: &str) {
  for (byte_index, character) in text.char_indices() {
    if character == ' ' && (byte_index == 0 || byte_index + 1 == text.len()) {
      markdown.push_str("&#32;");

      continue;
    }

    if character.is_control() {
      let _ = write!(markdown, "&#{};", u32::from(character));

      continue;
    }

    if matches!(
      character,
      '\\'
        | '`'
        | '*'
        | '_'
        | '['
        | ']'
        | '<'
        | '>'
        | '&'
        | '|'
        | '~'
        | '!'
        | '#'
        | '+'
        | '-'
        | '='
        | '.'
        | ')'
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
        push_escaped_text(&mut markdown, text);
        markdown.push('\n');
      }
      Node::Link { to, text } => {
        let label = text.as_deref().unwrap_or(to);

        if safe_link_target(to) {
          markdown.push('[');
          push_escaped_text(&mut markdown, label);
          markdown.push_str("](");
          push_escaped_destination(&mut markdown, to);
          markdown.push_str(")\n");
        } else {
          push_escaped_text(&mut markdown, label);
          markdown.push('\n');
        }
      }
      Node::Heading { level, text } => {
        markdown.push_str(match level {
          1 => "# ",
          2 => "## ",
          3 => "### ",
          _ => " ",
        });
        push_escaped_text(&mut markdown, text);
        markdown.push('\n');
      }
      Node::List(items) =>
        for item in items {
          markdown.push_str("- ");
          push_escaped_text(&mut markdown, item);
          markdown.push('\n');
        },
      Node::Blockquote(text) => {
        markdown.push_str("> ");
        push_escaped_text(&mut markdown, text);
        markdown.push('\n');
      }
      Node::PreformattedText { alt_text, text } => {
        let longest_tilde_run = text
          .split(|character| character != '~')
          .map(str::len)
          .max()
          .unwrap_or(0);
        let fence = "~".repeat((longest_tilde_run + 1).max(3));

        markdown.push_str(&fence);

        if let Some(alt_text) = alt_text {
          markdown.push(' ');
          markdown.extend(
            alt_text.chars().filter(|character| !character.is_control()),
          );
        }

        markdown.push('\n');
        markdown.push_str(text);

        if !text.is_empty() && !text.ends_with('\n') {
          markdown.push('\n');
        }

        markdown.push_str(&fence);
        markdown.push('\n');
      }
      Node::Whitespace => markdown.push('\n'),
    }
  }

  markdown
}
