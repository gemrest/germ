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

//! Convert Gemtext into many types of markup.

use crate::ast::Ast;

mod html;
mod markdown;

#[cfg(feature = "macros")] mod macros;

/// Different targets to convert Gemtext to
#[derive(Clone)]
pub enum Target {
  /// Convert Gemtext to HTML
  HTML,
  /// Convert Gemtext to Markdown
  Markdown,
}

/// Convert AST'd Gemtext into an alternative markup format.
///
/// # Example
///
/// ```rust
/// use germ::convert;
///
/// let _ = convert::from_ast(
///   &germ::ast::Ast::from_string(r#"=> gemini://gem.rest/ GemRest"#),
///   &convert::Target::HTML,
/// );
/// ```
#[must_use]
pub fn from_ast(source: &Ast, target: &Target) -> String {
  match target {
    Target::Markdown => markdown::convert(source.inner()),
    Target::HTML => html::convert(source.inner()),
  }
}

/// Convert raw Gemtext into an alternative markup format.
///
/// # Example
///
/// ```rust
/// use germ::convert;
///
/// let _ = convert::from_string(
///   r#"=> gemini://gem.rest/ GemRest"#,
///   &convert::Target::HTML,
/// );
/// ```
#[must_use]
pub fn from_string(
  source: &(impl ToString + ?Sized),
  target: &Target,
) -> String {
  from_ast(&Ast::from_owned(&source.to_string()), target)
}
