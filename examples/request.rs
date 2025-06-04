//! This example demonstrates Germ's capabilities for performing a non-blocking
//! request to a Gemini capsule.

#[tokio::main]
async fn main() {
  // Form a valid URL to a Gemini capsule
  let url = url::Url::parse("gemini://fuwn.me").unwrap();
  // Perform a non-blocking request to the Gemini capsule
  let request = germ::request::request(&url).await;

  match request {
    // If the request was successful, print a debug view of the response
    Ok(response) => {
      // Print the status of the response
      println!("{:?}", response.status());

      // Print the meta string of the response
      //
      // More detailed meta usage can be found in the `meta` example
      println!("{}", response.meta());

      // Print the content of the response, if present
      println!("{:?}", response.content());

      // Print the size of the response
      println!("{:?}", response.size());

      // Print a debug view of the SSL suite used
      println!("{:?}", response.suite());
    }
    // If the request was unsuccessful, do nothing
    Err(_) => {}
  }
}
