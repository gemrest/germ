#[cfg(test)]
mod test {
  use germ::quick::{self, heading};

  #[test]
  fn all_heading_levels() {
    assert_eq!(heading("Soup", &germ::quick::HeadingLevel::One), "# Soup");
    assert_eq!(
      heading("Vegetables", &germ::quick::HeadingLevel::Two),
      "## Vegetables"
    );
    assert_eq!(
      heading("Fruits", &germ::quick::HeadingLevel::Three),
      "### Fruits"
    );
  }

  #[test]
  fn list_item() {
    assert_eq!(quick::list_item("Soup"), "* Soup");
  }

  #[test]
  fn list_items() {
    assert_eq!(
      quick::list_items(&["Soup", "Vegetables", "Fruits"]),
      "* Soup\n* Vegetables\n* Fruits"
    );
  }

  #[test]
  fn link_variants() {
    assert_eq!(quick::link("Soup", None), "=> Soup");
    assert_eq!(
      quick::link("Soup", Some("gemini://soup.com")),
      "=> Soup gemini://soup.com"
    );
  }

  #[test]
  fn block_quote() {
    assert_eq!(quick::block_quote("Soup"), "> Soup");
  }

  #[test]
  fn preformatted_text_variants() {
    assert_eq!(quick::preformatted_text("Soup", None), "```\nSoup\n```");
    assert_eq!(
      quick::preformatted_text("Vegetables", Some("Fruits")),
      "```Fruits\nVegetables\n```"
    );
  }
}
