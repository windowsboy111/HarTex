/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

#![allow(incomplete_features)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(let_chains)]

use std::path::PathBuf;

use unic_langid::langid;
use unic_langid::LanguageIdentifier;
use hartex_eyre::eyre::Report;

pub mod types;

pub fn create_bundle(requested: Option<LanguageIdentifier>, modules: Vec<&str>) -> hartex_eyre::Result<types::LocalizationBundle> {
    let fallback = langid!("en-US");
    let is_fallback = requested.as_ref() == Some(&fallback);

    let locale = requested.clone().unwrap_or(fallback);
    let _ = types::LocalizationBundle::new(vec![locale]);

    let mut localizations_root = PathBuf::from("../localization/locales");
    if !is_fallback && let Some(ident) = requested {
        localizations_root.push(ident.to_string());
        modules.iter().for_each(|module| localizations_root.push(module));
    }

    if !localizations_root.try_exists()? {
        return Err(Report::msg(format!("localization root not found: {}", localizations_root.to_string_lossy())));
    }

    todo!()
}
