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
