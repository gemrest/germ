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
pub fn heading(text: &str, level: &HeadingLevel) -> String {
  format!(
    "{} {}",
    match level {
      HeadingLevel::One => "#",
      HeadingLevel::Two => "##",
      HeadingLevel::Three => "###",
    },
    text
  )
}

#[must_use]
pub fn list_item(text: &str) -> String { format!("* {}", text) }

#[must_use]
pub fn list_items(items: &[&str]) -> String {
  items
    .iter()
    .map(|item| list_item(item))
    .collect::<Vec<_>>()
    .join("\n")
}

#[must_use]
pub fn link(text: &str, location: Option<&str>) -> String {
  format!(
    "=> {}{}",
    text,
    location.map_or_else(|| "".to_string(), |l| format!(" {}", l))
  )
}

#[must_use]
pub fn block_quote(text: &str) -> String { format!("> {}", text) }

#[must_use]
pub fn preformatted_text(text: &str, alt_text: Option<&str>) -> String {
  format!("```{}\n{}\n```", alt_text.unwrap_or(""), text)
}
