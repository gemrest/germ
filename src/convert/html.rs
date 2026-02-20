use {crate::ast::Node, std::fmt::Write};

pub fn convert(source: &[Node]) -> String {
  let mut html = String::new();

  // Since we have an AST tree of the Gemtext, it is very easy to convert from
  // this AST tree to an alternative markup format.
  for node in source {
    match node {
      Node::Text(text) => {
        let _ = write!(&mut html, "<p>{text}</p>");
      }
      Node::Link { to, text } => {
        let _ = write!(
          &mut html,
          "<a href=\"{}\">{}</a><br>",
          to,
          text.clone().unwrap_or_else(|| to.clone())
        );
      }
      Node::Heading { level, text } => {
        let _ = write!(
          &mut html,
          "<{}>{}</{0}>",
          match level {
            1 => "h1",
            2 => "h2",
            3 => "h3",
            _ => "p",
          },
          text
        );
      }
      Node::List(items) => {
        let _ = write!(&mut html, "<ul>");

        for item in items {
          let _ = write!(&mut html, "<li>{item}</li>");
        }

        let _ = write!(&mut html, "</ul>");
      }
      Node::Blockquote(text) => {
        let _ = write!(&mut html, "<blockquote>{text}</blockquote>");
      }
      Node::PreformattedText { text, .. } => {
        let _ = write!(&mut html, "<pre>{text}</pre>");
      }
      Node::Whitespace => {}
    }
  }

  html
}
