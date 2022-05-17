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
  use germ::meta::Meta;

  #[test]
  fn meta_to_map_mime() {
    assert_eq!(
      Meta::from_str("text/gemini; hi=2; hi2=string=2").mime,
      "text/gemini",
    );
  }

  #[test]
  fn meta_to_map_with_parameters() {
    assert_eq!(
      Meta::from_str("text/gemini; hi=2; hi2=string=2")
        .parameters
        .get("hi2"),
      Some(&"string=2".to_string()),
    );
  }

  #[test]
  fn meta_to_map_length() {
    assert_eq!(
      Meta::from_str("text/gemini; hi=2; hi2=string=2")
        .parameters
        .len(),
      2,
    );
  }
}
