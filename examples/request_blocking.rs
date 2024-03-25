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

//! This example demonstrates Germ's capabilities for performing a blocking
//! request to a Gemini capsule.

fn main() {
  // Form a valid URL to a Gemini capsule
  let url = url::Url::parse("gemini://fuwn.me").unwrap();
  // Perform a blocking request to the Gemini capsule
  let request = germ::request::blocking::request(&url);

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
