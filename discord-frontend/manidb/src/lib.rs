/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # Database Manipulation Procedures
//!
//! The `manidb` crate implements database manipulation procedures for utilization in the
//! codebase.

#![deny(clippy::pedantic)]
#![deny(warnings)]

use std::future::Future;
use std::pin::Pin;

use base::error::Result;

pub mod whitelist;

/// A future that is pending and is yet to return.
pub type Pending<T> = Pin<Box<dyn Future<Output = Result<T>> + Send>>;