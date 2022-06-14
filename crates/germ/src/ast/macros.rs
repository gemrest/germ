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

//! Macros to aid with various Germ-related functionalities.

/// Convert a Gemini token tree into an `Ast`
///
/// # Example
///
/// ```rust
/// // Using a value
/// assert_eq!(
///   germ::gemini_to_ast!("=> / A link!").to_gemtext(),
///   // `to_gemtext` appends a newline to all responses, so let's make sure we
///   // account for that.
///   format!("{}\n", "=> / A link!"),
/// );
///
/// /// Using raw Gemtext
/// assert_eq!(
///   germ::gemini_to_ast! {
///     => / A link!
///     => / Another link!
///   }
///   .to_gemtext(),
///   format!("{}\n", "=> / A link!\n=> / Another link!"),
/// );
/// ```
#[macro_export]
macro_rules! gemini_to_ast {
  ($gemini:expr) => {
    germ::ast::Ast::from_string($gemini)
  };
  ($($gemini:tt)*) => {
    germ::ast::Ast::from_string(germ_macros_impl::gemini_to_tt!($($gemini)*));
  };
}
