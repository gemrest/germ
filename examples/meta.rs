//! This example demonstrates Germ's capabilities for parsing Gemini meta
//! sections.

fn main() {
  let meta = germ::meta::Meta::from_string(
    r#"text/gemini; charset=utf-8; title="two; parts""#,
  );

  println!("{:?}", meta);
  println!("{}", meta);
  println!("{}", meta.mime());
  println!("{:?}", meta.parameters());
}
