//! This example demonstrates a chain of Germ's capabilities by fetching a
//! Gemini capsule, parsing the response content into an abstract syntax tree,
//! and converting the abstract syntax tree back to Gemtext, identical to the
//! Gemini response content.

fn main() {
  // Form a valid URL to a Gemini capsule
  let url = url::Url::parse("gemini://fuwn.me/").unwrap();
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
      println!("{}", gemtext)
    }
    // If the request was unsuccessful, do nothing
    Err(_) => {}
  }
}
