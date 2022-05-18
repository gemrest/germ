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

use std::collections::HashMap;

/// Structure-ize a Gemini response's meta section into it's mime type and it's
/// parameters.
#[derive(Debug)]
pub struct Meta {
  /// The mime type of a Gemini response
  mime:       String,
  /// The parameters of a Gemini response
  parameters: HashMap<String, String>,
}
impl Meta {
  #[must_use]
  pub fn from_string(meta: &str) -> Self {
    let mut metas = meta.split(';');
    let mime = metas.next().unwrap_or("").to_string();
    let mut parameters = HashMap::new();

    for parameter in metas {
      let key_value = parameter
        .trim_start()
        .split_at(parameter.find('=').unwrap_or(0));

      parameters.insert(
        key_value.0.to_string().replace('=', ""),
        key_value.1.to_string(),
      );
    }

    Self {
      mime,
      parameters,
    }
  }

  #[must_use]
  pub fn mime(&self) -> &str { &self.mime }

  #[must_use]
  pub const fn parameters(&self) -> &HashMap<String, String> {
    &self.parameters
  }
}
