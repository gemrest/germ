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

#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code,
  clippy::all,
  clippy::nursery,
  clippy::pedantic
)]
#![recursion_limit = "128"]

#[cfg(feature = "ast")] pub mod ast;

#[cfg(feature = "convert")] pub mod convert;

#[cfg(feature = "request")] pub mod request;

#[cfg(feature = "meta")] pub mod meta;

#[cfg(feature = "quick")] pub mod quick;

#[cfg(feature = "example-gemtext")]
pub const EXAMPLE_GEMTEXT: &str = r"```This is alt-text
Here goes the pre-formatted text.

This continues the pre-formatted text on a new line after a blank line.
```

# This is a heading

This is some text.

This is more text after a blank line.

* This is a single list item.
* This is the next list item.

* This is a new list.
* This is the next item on the new list.

## This is a sub-heading

> This is a blockquote.

### This is a sub-sub-heading.

=> gemini://gem.rest/ This is a link to GemRest
=> /somewhere

```This is a preformatted block containing inner Gemtext.
=> gemini://fuwn.me/ This is a link.

* This is a list item.

> This is a blockquote.

# This is a heading.

## This is a sub-heading.

### This is a sub-sub-heading.
```

That was a link without text.";
