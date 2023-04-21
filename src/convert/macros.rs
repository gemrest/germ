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
