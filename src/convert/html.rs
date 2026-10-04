//! Customisable HTML rendering with escaped output by default.

use {
  super::safe_link_target,
  crate::ast::{Ast, Node},
};

/// Hooks for formatting content and rendering nodes with caller-owned state.
///
/// Germ supplies the standard HTML elements and link target safety checks.
/// Hook output is trusted HTML: implementations must escape untrusted content
/// themselves when overriding formatting or writing custom nodes.
/// Caller-supplied hooks are used only by this module's [`from_ast`] and
/// [`from_string`] functions.
/// The existing [`super::from_ast`] and [`super::from_string`] APIs always use
/// Germ's escaped defaults.
///
/// Implement only the hooks you need. The unit type `()` uses every default.
pub trait Renderer {
  /// Appends text or inline HTML inside a standard element.
  ///
  /// Called for text, headings, each list item, blockquotes and link labels,
  /// including labels of unsafe links and targets used as missing labels.
  /// The default escapes HTML special characters.
  fn format_text(&mut self, text: &str, html: &mut String) {
    push_escaped(html, text);
  }

  /// Appends content inside a standard `<pre>` element.
  ///
  /// The default escapes the entire text without removing any newlines. A
  /// caller can remove one trailing newline before delegating to the unit
  /// type's default implementation:
  ///
  /// ```rust
  /// use germ::convert::html::{self, Renderer};
  ///
  /// struct TrimPreformatted;
  ///
  /// impl Renderer for TrimPreformatted {
  ///   fn format_preformatted(&mut self, text: &str, html: &mut String) {
  ///     ().format_preformatted(text.strip_suffix('\n').unwrap_or(text), html);
  ///   }
  /// }
  ///
  /// assert_eq!(
  ///   html::from_string("```\n<&>\n```", &mut TrimPreformatted),
  ///   "<pre>&lt;&amp;&gt;</pre>",
  /// );
  /// ```
  fn format_preformatted(&mut self, text: &str, html: &mut String) {
    push_escaped(html, text);
  }

  /// Returns HTML inserted between list items, with no separator by default.
  /// Use `"\n"` to retain a caller's existing list formatting.
  fn list_item_separator(&self) -> &'static str { "" }

  /// Observes each node in order, including [`Node::Whitespace`], before any
  /// standard output or formatting for that node.
  ///
  /// Return `true` when the node has been handled, even if it emitted no HTML.
  /// Return `false` to append Germ's standard rendering after any output
  /// written by this hook. The default returns `false` without writing
  /// anything. Custom nodes bypass Germ's escaping and link target safety
  /// checks.
  fn render_node(&mut self, _node: &Node, _html: &mut String) -> bool { false }

  /// Appends final output after all nodes have been processed.
  /// Called exactly once, including for an empty AST. The default does nothing.
  fn finish(&mut self, _html: &mut String) {}
}

impl Renderer for () {}

/// Renders an AST with the supplied hooks and returns an HTML fragment.
///
/// The same renderer is used for every node and finalisation, so its state
/// remains available to the caller afterwards. Returning `false` from
/// [`Renderer::render_node`] delegates to Germ's standard rendering.
#[must_use]
pub fn from_ast(source: &Ast, renderer: &mut impl Renderer) -> String {
  render(source.inner(), renderer)
}

/// Parses Gemtext and renders it with the supplied hooks.
#[must_use]
pub fn from_string(
  source: impl AsRef<str>,
  renderer: &mut impl Renderer,
) -> String {
  from_ast(&Ast::from_string(source), renderer)
}

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

pub(super) fn convert(source: &[Node]) -> String { render(source, &mut ()) }

fn render(source: &[Node], renderer: &mut impl Renderer) -> String {
  let mut html = String::new();

  for node in source {
    if renderer.render_node(node, &mut html) {
      continue;
    }

    match node {
      Node::Text(text) => {
        html.push_str("<p>");
        renderer.format_text(text, &mut html);
        html.push_str("</p>");
      }

      Node::Link { to, text } => {
        let label = text.as_deref().unwrap_or(to);

        if safe_link_target(to) {
          html.push_str("<a href=\"");
          push_escaped(&mut html, to);
          html.push_str("\">");
          renderer.format_text(label, &mut html);
          html.push_str("</a><br>");
        } else {
          renderer.format_text(label, &mut html);
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
        renderer.format_text(text, &mut html);
        html.push_str("</");
        html.push_str(tag);
        html.push('>');
      }

      Node::List(items) => {
        html.push_str("<ul>");

        for (index, item) in items.iter().enumerate() {
          if index > 0 {
            html.push_str(renderer.list_item_separator());
          }

          html.push_str("<li>");
          renderer.format_text(item, &mut html);
          html.push_str("</li>");
        }

        html.push_str("</ul>");
      }

      Node::Blockquote(text) => {
        html.push_str("<blockquote>");
        renderer.format_text(text, &mut html);
        html.push_str("</blockquote>");
      }

      Node::PreformattedText { text, .. } => {
        html.push_str("<pre>");
        renderer.format_preformatted(text, &mut html);
        html.push_str("</pre>");
      }

      Node::Whitespace => {}
    }
  }

  renderer.finish(&mut html);

  html
}
