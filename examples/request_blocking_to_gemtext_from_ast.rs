//! This example demonstrates a chain of Germ's capabilities by fetching a
//! Gemini capsule, parsing the response content into an abstract syntax tree,
//! and converting the abstract syntax tree back to Gemtext, identical to the
//! Gemini response content.

use std::env;

fn main() {
  // Try to obtain a URL from the command line arguments or use the default one
  let url_string =
    env::args().nth(1).unwrap_or_else(|| "gemini://fuwn.me/".to_string());
  // Form a valid URL to a Gemini capsule
  let url = match url::Url::parse(&url_string) {
    Ok(url) => url,
    Err(error) => {
      eprintln!("Error parsing URL '{}': {}", url_string, error);
      std::process::exit(1);
    }
  };

  // Perform a blocking request to the Gemini capsule
  let request = germ::request::blocking::request(&url);

  match request {
    // If the request was successful:
    Ok(response) => {
      // Obtain the content of the Gemini response
      let response_content =
        &*response.content().clone().unwrap_or_else(|| "".to_string());
      // Parse the Gemini response content into an abstract syntax tree
      let ast = germ::ast::Ast::from_string(response_content);
      // Convert the abstract syntax tree back to Gemtext, identical to the
      // Gemini response content, constructed from the parsed abstract syntax
      // tree
      let gemtext = ast.to_gemtext();

      // Print the Gemtext
      println!("{}", gemtext);

      // Check if the response content and reconstruction are identical
      if response_content == gemtext {
        println!(
          "\nValidation: Response content and reconstruction are identical"
        );
      } else {
        println!("\nValidation: Response content and reconstruction differ");
        print_diff(response_content, &gemtext);
      }
    }
    // If the request was unsuccessful, print an error message and exit
    Err(error) => {
      eprintln!("Error fetching '{}': {}", url_string, error);
      std::process::exit(1);
    }
  }
}

fn print_diff(original: &str, reconstructed: &str) {
  use std::io::{self, Write};

  let mut stdout = io::stdout();
  let original_lines = original.lines().collect::<Vec<&str>>();
  let reconstructed_lines = reconstructed.lines().collect::<Vec<&str>>();
  let max_lines = original_lines.len().max(reconstructed_lines.len());
  let mut has_printed_diff = false;

  for i in 0..max_lines {
    let original_line = original_lines.get(i).unwrap_or(&"");
    let reconstructed_line = reconstructed_lines.get(i).unwrap_or(&"");

    if original_line != reconstructed_line {
      if has_printed_diff {
        let _ = writeln!(stdout);
      }

      let _ = writeln!(stdout, "Line {}:", i + 1);
      let _ = writeln!(stdout, "  Original:      '{}'", original_line);
      let _ = writeln!(stdout, "  Reconstructed: '{}'", reconstructed_line);

      has_printed_diff = true;
    }
  }

  if original_lines.len() != reconstructed_lines.len() {
    if has_printed_diff {
      let _ = writeln!(stdout);
    }

    let _ = writeln!(stdout, "Length difference:");
    let _ = writeln!(stdout, "  Original:      {} lines", original_lines.len());
    let _ =
      writeln!(stdout, "  Reconstructed: {} lines", reconstructed_lines.len());
  }
}
