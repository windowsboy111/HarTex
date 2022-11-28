/*
 * SPDX-License-Identifier: AGPL-3.0-only
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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use proc_macro::{Span, TokenStream, TokenTree};

use crate::internal::StreamParser;

#[derive(Debug)]
pub enum DeriveAttribute {
    IdType(TokenTree),
}

#[derive(Debug)]
pub struct DeriveStream {
    pub attributes: Vec<DeriveAttribute>,
    pub identifier: Option<TokenTree>,
}

impl DeriveStream {
    fn new() -> Self {
        Self {
            attributes: Vec::new(),
            identifier: None,
        }
    }
}

impl StreamParser for DeriveStream {
    fn parse(tokens: TokenStream) -> Option<Self> {
        let _ = DeriveStream::new();

        let iter = tokens.into_iter().peekable();

        if !iter
            .clone()
            .any(|tree| tree.to_string() == String::from("pub"))
        {
            Span::call_site()
                .error("entitycache traits can only be derived on pub items")
                .emit();
            return None;
        }

        todo!()
    }
}