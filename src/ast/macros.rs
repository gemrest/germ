/// Convert Gemtext an `Ast`
///
/// # Examples
///
/// ```rust
/// // Using a value
/// assert_eq!(
///   germ::gemini_to_ast!("=> / A link!").to_gemtext(),
///   // `to_gemtext` appends a newline to all responses, so let's make sure we
///   // account for that.
///   "=> / A link!",
/// );
#[macro_export]
macro_rules! gemini_to_ast {
  ($gemini:expr) => {
    $crate::ast::Ast::from_string($gemini)
  };
}
