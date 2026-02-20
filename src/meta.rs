use std::{borrow::Cow, collections::HashMap, fmt::Display};

/// Structure-ize a Gemini response's meta section into it's mime type and it's
/// parameters.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Meta {
  /// The mime type of a Gemini response
  mime:       String,
  /// The parameters of a Gemini response
  parameters: HashMap<String, String>,
}

impl Display for Meta {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.mime)?;

    if !self.parameters.is_empty() {
      write!(f, "; ")?;

      let mut parameters: Vec<_> = self.parameters.iter().collect();

      parameters.sort_by(|a, b| a.0.cmp(b.0));

      for (i, (key, value)) in parameters.iter().enumerate() {
        if i > 0 {
          write!(f, "; ")?;
        }

        write!(f, "{key}={value}")?;
      }
    }

    Ok(())
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
      let trimmed = parameter.trim_start();

      // Only parse parameters containing '=' as those without are malformed
      // according to RFC 2045
      if let Some(equal_pos) = trimmed.find('=') {
        let (key, value) = trimmed.split_at(equal_pos);

        parameters.insert(key.to_string(), value[1..].to_string());
      }
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
  #[allow(clippy::missing_const_for_fn)]
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
  pub const fn mime_mut(&mut self) -> &mut String { &mut self.mime }

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
  pub const fn parameters_mut(&mut self) -> &mut HashMap<String, String> {
    &mut self.parameters
  }
}
