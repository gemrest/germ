//! This example demonstrates Germ's capabilities for performing a blocking
//! request to a Gemini capsule.

fn main() -> anyhow::Result<()> {
  let url = url::Url::parse("gemini://fuwn.me")?;
  let response = germ::request::blocking::request(&url)?;

  println!("{:?}", response.status());
  println!("{}", response.meta());
  println!("{:?}", response.content());
  println!("{:?}", response.size());
  println!("{:?}", response.suite());

  Ok(())
}
