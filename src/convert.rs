//! Convert Gemtext into many types of markup.

use crate::ast::Ast;

mod html;
mod markdown;

#[cfg(feature = "macros")] mod macros;

/// Different targets to convert Gemtext to
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Target {
  /// Convert Gemtext to HTML. Links outside the Gemini, Gopher, HTTP, HTTPS,
  /// mailto, and FTP schemes are rendered as text.
  HTML,
  /// Convert Gemtext to Markdown
  Markdown,
}

/// Convert AST'd Gemtext into an alternative markup format.
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

/// Convert raw Gemtext into an alternative markup format.
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
