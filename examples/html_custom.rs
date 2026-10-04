//! Customise inline text, preformatted newlines and adjacent link groups.
//! Germ still renders standard elements and checks link targets. Returning
//! `true` handles a node; returning `false` delegates to standard rendering.

use germ::{
  ast::Node,
  convert::html::{self, Renderer},
};

#[derive(Default)]
struct LinkNavigation {
  link_group_open: bool,
}

impl LinkNavigation {
  fn close_link_group(&mut self, html: &mut String) {
    if self.link_group_open {
      html.push_str("</nav>");

      self.link_group_open = false;
    }
  }
}

impl Renderer for LinkNavigation {
  fn format_text(&mut self, text: &str, html: &mut String) {
    html.push_str("<em>");
    ().format_text(text, html);
    html.push_str("</em>");
  }

  fn format_preformatted(&mut self, text: &str, html: &mut String) {
    ().format_preformatted(text.strip_suffix('\n').unwrap_or(text), html);
  }

  fn list_item_separator(&self) -> &'static str { "\n" }

  fn render_node(&mut self, node: &Node, html: &mut String) -> bool {
    if matches!(node, Node::Link { .. }) {
      if !self.link_group_open {
        html.push_str("<nav>");

        self.link_group_open = true;
      }

      return false;
    }

    self.close_link_group(html);

    if matches!(node, Node::Whitespace) {
      html.push_str("<hr>");

      return true;
    }

    false
  }

  fn finish(&mut self, html: &mut String) { self.close_link_group(html); }
}

fn main() {
  let source = r#"# Custom HTML
Text with <tags> & quotes.
* First <item>
* Second item
> A quote
```Code
<code>
```
=> /one First link
=> /two Second link

=> javascript:alert(1) Unsafe target
# Next section
=> /three Final link"#;
  let mut renderer = LinkNavigation::default();
  let html = html::from_string(source, &mut renderer);

  println!("{html}");
}
