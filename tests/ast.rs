// This file is part of Germ <https://github.com/gemrest/germ>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod test {
  use germ::ast::{Ast, Node};

  #[test]
  fn build_multi_line_list_with_text() {
    assert_eq!(
      *Ast::from_string("* item1\n* 2\nhi text").inner(),
      vec![
        Node::List(vec!["item1".to_string(), "2".to_string()]),
        Node::Text("hi text".to_string()),
      ],
    );
  }

  #[test]
  fn build_multi_line_vec() {
    assert_eq!(
      *Ast::from_string("=> /test hi\nhi there\n> hi").inner(),
      vec![
        Node::Link {
          to:   "/test".to_string(),
          text: Some("hi".to_string()),
        },
        Node::Text("hi there".to_string()),
        Node::Blockquote("hi".to_string()),
      ],
    );
  }

  #[test]
  fn build_single_0th_from_vec() {
    assert_eq!(
      Ast::from_string("=> /test hi").inner(),
      &vec![Node::Link {
        to:   "/test".to_string(),
        text: Some("hi".to_string()),
      }],
    );
  }

  #[test]
  fn build_single_element() {
    assert_eq!(
      Ast::from_string("=> /test hi").inner().get(0).unwrap(),
      &Node::Link {
        to:   "/test".to_string(),
        text: Some("hi".to_string()),
      },
    );
  }
}
