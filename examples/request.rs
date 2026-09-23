//! This example demonstrates Germ's capabilities for performing a non-blocking
//! request to a Gemini capsule.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let url = url::Url::parse("gemini://fuwn.me")?;
  let response = germ::request::request(&url).await?;

  println!("{:?}", response.status());
  println!("{}", response.meta());
  println!("{:?}", response.content());
  println!("{:?}", response.size());
  println!("{:?}", response.suite());

  Ok(())
}
