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
  items
    .iter()
    .map(|item| list_item(&item.to_string()))
    .collect::<Vec<_>>()
    .join("\n")
}

#[must_use]
pub fn link(text: &(impl ToString + ?Sized), location: Option<&str>) -> String {
  format!(
    "=> {}{}",
    text.to_string(),
    location.map_or_else(String::new, |l| format!(" {l}"))
  )
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
