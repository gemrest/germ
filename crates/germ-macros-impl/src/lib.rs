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

#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code,
  clippy::all,
  clippy::nursery,
  clippy::pedantic
)]
#![feature(proc_macro_hygiene, proc_macro_span)]
#![recursion_limit = "128"]

use proc_macro::TokenStream;

/// Convert Gemtext into a token tree
///
/// # Panics
///
/// May panic if the Gemini could not be properly handled, for any reason.
#[proc_macro]
pub fn gemini_to_tt(input: TokenStream) -> TokenStream {
  let mut tokens = input.into_iter();
  let mut span = tokens.next().unwrap().span();

  for token in tokens {
    span = span.join(token.span()).unwrap();
  }

  let gemini = span
    .source_text()
    .unwrap()
    .lines()
    .map(|l| l.trim_start().to_string())
    .collect::<Vec<String>>()
    .join("\n");

  quote::quote!(#gemini).into()
}
