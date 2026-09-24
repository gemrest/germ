pub enum HeadingLevel {
  One,
  Two,
  Three,
}

#[must_use]
pub fn heading(
  text: &(impl ToString + ?Sized),
  level: &HeadingLevel,
) -> String {
  format!(
    "{} {}",
    match level {
      HeadingLevel::One => "#",
      HeadingLevel::Two => "##",
      HeadingLevel::Three => "###",
    },
    text.to_string()
  )
}

#[must_use]
pub fn list_item(text: &(impl ToString + ?Sized)) -> String {
  format!("* {}", text.to_string())
}

#[must_use]
pub fn list_items(items: &[&(impl ToString + ?Sized)]) -> String {
  let mut gemtext = String::new();

  for (index, item) in items.iter().enumerate() {
    if index > 0 {
      gemtext.push('\n');
    }

    gemtext.push_str("* ");
    gemtext.push_str(&item.to_string());
  }

  gemtext
}

/// Builds a link line with a URL and an optional label.
#[must_use]
pub fn link(url: &(impl ToString + ?Sized), label: Option<&str>) -> String {
  let mut gemtext = String::from("=> ");

  gemtext.push_str(&url.to_string());

  if let Some(label) = label {
    gemtext.push(' ');
    gemtext.push_str(label);
  }

  gemtext
}

#[must_use]
pub fn block_quote(text: &(impl ToString + ?Sized)) -> String {
  format!("> {}", text.to_string())
}

#[must_use]
pub fn preformatted_text(
  text: &(impl ToString + ?Sized),
  alt_text: Option<&str>,
) -> String {
  format!("```{}\n{}\n```", alt_text.unwrap_or(""), text.to_string())
}
