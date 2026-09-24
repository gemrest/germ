use std::{borrow::Cow, collections::HashMap, fmt::Display};

const MIME_SPECIAL_CHARACTERS: &[u8] = b"()<>@,;:\\\"/[]?=";

fn is_mime_token(value: &str) -> bool {
  !value.is_empty()
    && value.bytes().all(|byte| {
      byte.is_ascii_graphic() && !MIME_SPECIAL_CHARACTERS.contains(&byte)
    })
}

fn parse_parameter(parameter: &str, parameters: &mut HashMap<String, String>) {
  let Some((name, value)) = parameter.trim().split_once('=') else {
    return;
  };
  let name = name.trim();
  let value = value.trim();

  if !is_mime_token(name) {
    return;
  }

  let value = if let Some(quoted) = value.strip_prefix('"') {
    let Some(quoted) = quoted.strip_suffix('"') else {
      return;
    };
    let mut unescaped = String::with_capacity(quoted.len());
    let mut characters = quoted.chars();

    while let Some(character) = characters.next() {
      if character == '\\' {
        let Some(escaped) = characters.next() else {
          return;
        };

        unescaped.push(escaped);
      } else if character == '"' {
        return;
      } else {
        unescaped.push(character);
      }
    }

    unescaped
  } else {
    value.to_owned()
  };

  parameters.insert(name.to_owned(), value);
}

/// Structures a Gemini response's MIME type and parameters.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Meta {
  /// The MIME type of a Gemini response.
  mime:       String,
  /// The parameters of a Gemini response.
  parameters: HashMap<String, String>,
}

impl Display for Meta {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.mime)?;

    if !self.parameters.is_empty() {
      write!(f, "; ")?;

      let mut sorted_parameters: Vec<_> = self.parameters.iter().collect();

      sorted_parameters.sort_by(|left, right| left.0.cmp(right.0));

      for (index, (key, value)) in sorted_parameters.iter().enumerate() {
        if index > 0 {
          write!(f, "; ")?;
        }

        write!(f, "{key}=")?;

        if is_mime_token(value) {
          write!(f, "{value}")?;
        } else {
          write!(f, "\"")?;

          for character in value.chars() {
            if matches!(character, '"' | '\\') {
              write!(f, "\\")?;
            }

            write!(f, "{character}")?;
          }

          write!(f, "\"")?;
        }
      }
    }

    Ok(())
  }
}

impl Meta {
  /// Creates an empty `Meta`.
  ///
  /// # Example
  ///
  /// ```rust
  /// let meta = germ::meta::Meta::new();
  /// assert!(meta.parameters().is_empty());
  /// ```
  #[must_use]
  pub fn new() -> Self { Self::default() }

  /// Parses a MIME type and its semicolon-separated parameters.
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
    let meta = meta.into();
    let (mime, parameters_source) = meta.split_once(';').unwrap_or((&meta, ""));
    let mime = mime.to_owned();
    let mut parameters = HashMap::new();
    let mut parameter_start = 0;
    let mut quoted = false;
    let mut escaped = false;

    for (index, character) in parameters_source.char_indices() {
      if escaped {
        escaped = false;

        continue;
      }

      match character {
        '\\' if quoted => escaped = true,
        '"' => quoted = !quoted,
        ';' if !quoted => {
          parse_parameter(
            &parameters_source[parameter_start..index],
            &mut parameters,
          );

          parameter_start = index + 1;
        }
        _ => {}
      }
    }

    if !quoted {
      parse_parameter(&parameters_source[parameter_start..], &mut parameters);
    }

    Self { mime, parameters }
  }

  /// Returns the MIME type.
  ///
  /// # Example
  ///
  /// ```rust
  /// assert_eq!(
  ///   germ::meta::Meta::from_string("text/gemini; hi=2; hi2=string=2").mime(),
  ///   "text/gemini",
  /// );
  /// ```
  #[allow(clippy::missing_const_for_fn)]
  #[must_use]
  pub fn mime(&self) -> Cow<'_, str> { Cow::Borrowed(&self.mime) }

  /// Returns mutable access to the MIME type.
  ///
  /// # Example
  ///
  /// ```rust
  /// let mut meta = germ::meta::Meta::new();
  ///
  /// *meta.mime_mut() = "text/gemini".to_string();
  /// ```
  pub const fn mime_mut(&mut self) -> &mut String { &mut self.mime }

  /// Returns the parameters.
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

  /// Returns mutable access to the parameters.
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
  pub const fn parameters_mut(&mut self) -> &mut HashMap<String, String> {
    &mut self.parameters
  }
}
