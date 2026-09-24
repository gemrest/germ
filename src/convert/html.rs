use {super::safe_link_target, crate::ast::Node};

fn push_escaped(html: &mut String, value: &str) {
  for character in value.chars() {
    match character {
      '&' => html.push_str("&amp;"),
      '<' => html.push_str("&lt;"),
      '>' => html.push_str("&gt;"),
      '"' => html.push_str("&quot;"),
      '\'' => html.push_str("&#39;"),
      _ => html.push(character),
    }
  }
}

pub fn convert(source: &[Node]) -> String {
  let mut html = String::new();

  for node in source {
    match node {
      Node::Text(text) => {
        html.push_str("<p>");
        push_escaped(&mut html, text);
        html.push_str("</p>");
      }

      Node::Link { to, text } => {
        let label = text.as_deref().unwrap_or(to);

        if safe_link_target(to) {
          html.push_str("<a href=\"");
          push_escaped(&mut html, to);
          html.push_str("\">");
          push_escaped(&mut html, label);
          html.push_str("</a><br>");
        } else {
          push_escaped(&mut html, label);
          html.push_str("<br>");
        }
      }

      Node::Heading { level, text } => {
        let tag = match level {
          1 => "h1",
          2 => "h2",
          3 => "h3",
          _ => "p",
        };

        html.push('<');
        html.push_str(tag);
        html.push('>');
        push_escaped(&mut html, text);
        html.push_str("</");
        html.push_str(tag);
        html.push('>');
      }

      Node::List(items) => {
        html.push_str("<ul>");

        for item in items {
          html.push_str("<li>");
          push_escaped(&mut html, item);
          html.push_str("</li>");
        }

        html.push_str("</ul>");
      }

      Node::Blockquote(text) => {
        html.push_str("<blockquote>");
        push_escaped(&mut html, text);
        html.push_str("</blockquote>");
      }

      Node::PreformattedText { text, .. } => {
        html.push_str("<pre>");
        push_escaped(&mut html, text);
        html.push_str("</pre>");
      }

      Node::Whitespace => {}
    }
  }

  html
}
