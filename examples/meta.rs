//! This example demonstrates Germ's capabilities for parsing Gemini meta
//! sections.

fn main() {
  // Parse Gemini meta section into a structured meta representation
  let meta = germ::meta::Meta::from_string("text/gemini; hi=2; hi2=string=2");

  // Debug view of the structured meta representation
  println!("{:?}", meta);

  // Convert the structured meta representation back to a string, identical to
  // the original meta section
  println!("{}", meta);

  // The MIME type of the meta section
  println!("{}", meta.mime());

  // A debug view of the parameters of the meta section
  println!("{:?}", meta.parameters());
}
