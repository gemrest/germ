//! Fetches a Gemtext response and checks its AST round trip.
//! This example's TOFU certificate store lasts only for one run.

use {
  germ::request::{CertificateStore, RequestOptions},
  std::{collections::HashMap, env},
};

#[derive(Default)]
struct MemoryCertificates(HashMap<(String, u16), Vec<u8>>);

impl CertificateStore for MemoryCertificates {
  fn load(
    &mut self,
    hostname: &str,
    port: u16,
  ) -> anyhow::Result<Option<Vec<u8>>> {
    Ok(self.0.get(&(hostname.to_owned(), port)).cloned())
  }

  fn save(
    &mut self,
    hostname: &str,
    port: u16,
    certificate: &[u8],
  ) -> anyhow::Result<()> {
    self.0.insert((hostname.to_owned(), port), certificate.to_vec());

    Ok(())
  }
}

fn main() -> anyhow::Result<()> {
  let url_string = env::args()
    .nth(1)
    .unwrap_or_else(|| "gemini://geminiprotocol.net/".to_owned());
  let url = url::Url::parse(&url_string)?;
  let mut certificates = MemoryCertificates::default();
  let response = germ::request::blocking::request_with_tofu(
    &url,
    &mut certificates,
    &RequestOptions::default(),
  )?;

  anyhow::ensure!(
    response.status().category()
      == Some(germ::request::StatusCategory::Success),
    "Gemini response was not successful"
  );

  let response_meta = response.meta();
  let mime_type = response_meta.split(';').next().unwrap_or_default().trim();

  anyhow::ensure!(
    mime_type.eq_ignore_ascii_case("text/gemini"),
    "Gemini response is not Gemtext"
  );

  let response_bytes = response.content_bytes().unwrap_or_default();
  let original = std::str::from_utf8(response_bytes)?;
  let ast = germ::ast::Ast::from_string(original);
  let reconstructed = ast.to_gemtext();
  let reparsed = germ::ast::Ast::from_string(&reconstructed);

  anyhow::ensure!(
    ast.inner() == reparsed.inner(),
    "Gemtext reconstruction changed the parsed nodes"
  );

  if original != reconstructed {
    eprintln!("Gemtext source formatting was normalised:");
    print_diff(original, &reconstructed);
  } else {
    eprintln!("Gemtext reconstruction matches the response exactly");
  }

  print!("{reconstructed}");

  Ok(())
}

fn print_diff(original: &str, reconstructed: &str) {
  let mut original_lines = original.split('\n');
  let mut reconstructed_lines = reconstructed.split('\n');
  let mut line_number = 1;

  loop {
    let original_line = original_lines.next();
    let reconstructed_line = reconstructed_lines.next();

    if original_line.is_none() && reconstructed_line.is_none() {
      break;
    }

    if original_line != reconstructed_line {
      eprintln!(
        "Line {line_number}: original {original_line:?}, reconstructed \
         {reconstructed_line:?}"
      );
    }

    line_number += 1;
  }
}
