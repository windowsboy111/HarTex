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

use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::util::ImageHash;

pub struct Cdn;

impl Cdn {
    pub const URL_BASE: &'static str = "https://cdn.discordapp.com/";

    pub fn guild_icon(guild_id: Id<GuildMarker>, icon: ImageHash) -> String {
        let mut url = format!("{URL_BASE}icons/{guild_id}/{icon}");
        if icon.is_animated() {
            url.push_str(".gif");
        } else {
            url.push_str(".png");
        }

        url
    }
}