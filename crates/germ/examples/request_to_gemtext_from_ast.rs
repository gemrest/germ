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

fn main() {
  match germ::request::request(&url::Url::parse("gemini://fuwn.me/").unwrap()) {
    Ok(response) =>
      println!(
        "{}",
        germ::ast::Ast::from_string(
          &*response.content().clone().unwrap_or_else(|| "".to_string())
        )
        .to_gemtext()
      ),
    Err(_) => {}
  }
}
