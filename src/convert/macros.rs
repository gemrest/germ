/// Convert Gemtext into HTML
///
/// # Examples
///
/// ```rust
/// // Using a value
/// assert_eq!(
///   germ::gemini_to_html!("=> /to hello !"),
///   "<a href=\"/to\">hello !</a><br>",
/// );
#[macro_export]
macro_rules! gemini_to_html {
  ($gemini:expr) => {
    $crate::convert::from_ast(
      &$crate::gemini_to_ast!($gemini),
      &$crate::convert::Target::HTML,
    )
  };
  ($($gemini:tt)*) => {
    $crate::convert::from_ast(
      &$crate::gemini_to_ast!{ $($gemini)* },
      &$crate::convert::Target::HTML,
    )
  };
}

/// Convert Gemtext into Markdown
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///   // Using a value
///   germ::gemini_to_md!("=> /to hello !"),
///   "[hello !](/to)\n",
/// );
#[macro_export]
macro_rules! gemini_to_md {
  ($gemini:expr) => {
    $crate::convert::from_ast(
      &$crate::gemini_to_ast!($gemini),
      &$crate::convert::Target::Markdown,
    )
  };
  ($($gemini:tt)*) => {
    $crate::convert::from_ast(
      &$crate::gemini_to_ast!{ $($gemini)* },
      &$crate::convert::Target::Markdown,
    )
  };
}
