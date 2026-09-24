/// Converts Gemtext into an `Ast`.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///   germ::gemini_to_ast!("=> / A link!").to_gemtext(),
///   "=> / A link!",
/// );
/// ```
#[macro_export]
macro_rules! gemini_to_ast {
  ($gemini:expr) => {
    $crate::ast::Ast::from_string($gemini)
  };
}
