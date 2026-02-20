#[cfg(test)]
mod test {
  use germ::{
    convert::{Target, from_string},
    gemini_to_html, gemini_to_md,
  };

  #[test]
  fn convert_from_string_to_html_single_line() {
    assert_eq!(from_string("hi", &Target::HTML), "<p>hi</p>",);
  }

  #[test]
  fn convert_from_string_to_html_multi_line() {
    assert_eq!(from_string("hi\n# hi", &Target::HTML), "<p>hi</p><h1>hi</h1>",);
  }

  #[test]
  fn convert_from_string_to_html_single_link_macro_expression() {
    assert_eq!(
      gemini_to_html!("=> /to hello !"),
      "<a href=\"/to\">hello !</a><br>",
    );
  }

  #[test]
  fn convert_from_string_to_markdown_single_line() {
    assert_eq!(from_string("hi", &Target::Markdown), "hi\n",);
  }

  #[test]
  fn convert_from_string_to_markdown_multi_line() {
    assert_eq!(from_string("hi\n# hi", &Target::Markdown), "hi\n# hi\n",);
  }

  #[test]
  fn convert_from_string_to_markdown_single_link() {
    assert_eq!(
      from_string("=> /to hello !", &Target::Markdown),
      "[hello !](/to)\n",
    );
  }

  #[test]
  fn convert_from_string_to_markdown_single_macro_expression() {
    assert_eq!(gemini_to_md!("=> /to hello !"), "[hello !](/to)\n",);
  }
}
