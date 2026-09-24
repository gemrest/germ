//! Fetches a Gemtext response and checks its AST round trip.

use std::env;

fn main() -> anyhow::Result<()> {
  let url_string =
    env::args().nth(1).unwrap_or_else(|| "gemini://fuwn.me/".to_owned());
  let url = url::Url::parse(&url_string)?;
  let response = germ::request::blocking::request(&url)?;

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
  let reconstructed = germ::ast::Ast::from_owned(original).to_gemtext();

  print!("{reconstructed}");

  if original != reconstructed {
    print_diff(original, &reconstructed);
    anyhow::bail!("Gemtext reconstruction differs from the response");
  }

  eprintln!("Gemtext reconstruction matches the response");

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
