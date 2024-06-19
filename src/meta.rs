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

use std::{borrow::Cow, collections::HashMap, fmt::Display};

/// Structure-ize a Gemini response's meta section into it's mime type and it's
/// parameters.
#[derive(Debug, Default, Clone)]
pub struct Meta {
  /// The mime type of a Gemini response
  mime:       String,
  /// The parameters of a Gemini response
  parameters: HashMap<String, String>,
}

impl Display for Meta {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.mime, {
      let mut parameters = self
        .parameters
        .iter()
        .map(|(k, v)| format!("{}={}", *k, v))
        .collect::<Vec<_>>();

      parameters.sort();
      parameters.reverse();

      if parameters.is_empty() {
        String::new()
      } else {
        format!("; {}", parameters.join("; "))
      }
    })
  }
}

impl Meta {
  /// Create a new `Meta`
  ///
  /// # Example
  ///
  /// ```rust
  /// let mut meta = germ::meta::Meta::new(); 
  /// ```
  #[must_use]
  pub fn new() -> Self { Self::default() }

  /// Create a `Meta` from a string
  ///
  /// # Example
  ///
  /// ```rust
  /// assert_eq!(
  ///   germ::meta::Meta::from_string("text/gemini; hi=2; hi2=string=2").mime(),
  ///   "text/gemini",
  /// );
  /// ```
  #[must_use]
  pub fn from_string<'a>(meta: impl Into<std::borrow::Cow<'a, str>>) -> Self {
    let meta = meta.into().to_string();
    let mut metas = meta.split(';');
    let mime = metas.next().unwrap_or("").to_string();
    let mut parameters = HashMap::new();

    for parameter in metas {
      let key_value =
        parameter.trim_start().split_at(parameter.find('=').unwrap_or(0));

      parameters.insert(
        key_value.0.to_string().replace('=', ""),
        key_value.1.to_string(),
      );
    }

    Self { mime, parameters }
  }

  /// Obtain non-mutable access to the mime of the `Meta`
  ///
  /// # Example
  ///
  /// ```rust
  /// assert_eq!(
  ///   germ::meta::Meta::from_string("text/gemini; hi=2; hi2=string=2").mime(),
  ///   "text/gemini",
  /// );
  /// ```
  #[must_use]
  pub fn mime(&self) -> Cow<'_, str> { Cow::Borrowed(&self.mime) }

  /// Obtain mutable access to the mime of the `Meta`
  ///
  /// # Example
  ///
  /// ```rust
  /// let mut meta = germ::meta::Meta::new();
  ///
  /// *meta.mime_mut() = "text/gemini".to_string();
  /// ```
  pub fn mime_mut(&mut self) -> &mut String { &mut self.mime }

  /// Obtain non-mutable access to the parameters of the `Meta`
  ///
  /// # Example
  ///
  /// ```rust
  /// assert_eq!(
  ///   germ::meta::Meta::from_string("text/gemini; hi=2; hi2=string=2")
  ///     .parameters()
  ///     .get("hi2"),
  ///   Some(&"string=2".to_string()),
  /// );
  /// ```
  #[must_use]
  pub const fn parameters(&self) -> &HashMap<String, String> {
    &self.parameters
  }

  /// Obtain mutable access to the parameters of the `Meta`
  ///
  /// # Example
  ///
  /// ```rust
  /// let mut meta = germ::meta::Meta::new();
  /// let mut parameters = std::collections::HashMap::new();
  ///
  /// parameters.insert("hi".to_string(), "2".to_string());
  /// parameters.insert("hi2".to_string(), "string=2".to_string());
  ///
  /// *meta.parameters_mut() = parameters;
  /// ```
  pub fn parameters_mut(&mut self) -> &mut HashMap<String, String> {
    &mut self.parameters
  }
}
