//! Convert Gemtext into many types of markup.

use crate::ast::Ast;

mod html;
mod markdown;

#[cfg(feature = "macros")] mod macros;

fn safe_link_target(target: &str) -> bool {
  if target.is_empty()
    || target.chars().any(|character| {
      character.is_whitespace() || character.is_control() || character == '\\'
    })
  {
    return false;
  }

  match target.find([':', '/', '?', '#']) {
    Some(index) if target.as_bytes()[index] == b':' => matches!(
      target[..index].to_ascii_lowercase().as_str(),
      "gemini" | "gopher" | "http" | "https" | "mailto" | "ftp"
    ),
    _ => true,
  }
}

/// The available Gemtext conversion targets.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Target {
  /// Converts Gemtext to HTML. Links outside the Gemini, Gopher, HTTP, HTTPS,
  /// mailto, and FTP schemes are rendered as text.
  HTML,
  /// Converts Gemtext to Markdown with the same link scheme policy as HTML.
  Markdown,
}

/// Converts a Gemtext AST into another markup format.
///
/// # Example
///
/// ```rust
/// use germ::convert;
///
/// let _ = convert::from_ast(
///   &germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#),
///   &convert::Target::HTML,
/// );
/// ```
#[must_use]
pub fn from_ast(source: &Ast, target: &Target) -> String {
  match target {
    Target::Markdown => markdown::convert(source.inner()),
    Target::HTML => html::convert(source.inner()),
  }
}

/// Converts raw Gemtext into another markup format.
///
/// # Example
///
/// ```rust
/// use germ::convert;
///
/// let _ = convert::from_string(
///   r#"=> gemini://gem.rest/ GemRest"#,
///   &convert::Target::HTML,
/// );
/// ```
#[must_use]
pub fn from_string(
  source: &(impl ToString + ?Sized),
  target: &Target,
) -> String {
  from_ast(&Ast::from_value(source), target)
}
