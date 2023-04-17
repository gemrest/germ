// This file is part of Germ <https://github.com/gemrest/germ>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

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
