// This file is part of Germ <https://github.com/gemrest/germ>.
// Copyright (C) 2022-2023 Fuwn <contact@fuwn.me>
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

//! Make Gemini requests and get sane, structured results

mod response;
mod status;
mod verifier;

#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(feature = "request")]
pub mod non_blocking;

#[cfg(feature = "request")]
pub use non_blocking::request;

pub(crate) use verifier::GermVerifier;
pub use {response::Response, status::Status};
