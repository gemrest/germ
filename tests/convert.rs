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
  use germ::{
    convert::{from_string, Target},
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
