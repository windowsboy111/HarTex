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

//! Guild whitelist manipulation procedures.

use std::env;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use base::discord::model::id::{marker::GuildMarker, Id};
use base::error::{Error, Result};
use model::db::whitelist::WhitelistedGuild;

use crate::Pending;

pub struct GetGuildWhitelistStatus {
    future: Option<Pending<Option<WhitelistedGuild>>>,
    guild_id: Id<GuildMarker>,
}

impl GetGuildWhitelistStatus {
    pub fn new(guild_id: Id<GuildMarker>) -> Self {
        Self {
            future: None,
            guild_id,
        }
    }

    fn launch(&mut self) {
        log::trace!("launching future `GetGuildWhitelistStatus`");

        self.future.replace(Box::pin(exec(self.guild_id)));
    }
}

impl Future for GetGuildWhitelistStatus {
    type Output = Result<Option<WhitelistedGuild>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            self.launch();

            if let Some(future) = self.future.as_mut() {
                return future.as_mut().poll(cx);
            }
        }
    }
}

async fn exec(guild_id: Id<GuildMarker>) -> Result<Option<WhitelistedGuild>> {
    let db_credentials = env::var("PGSQL_WHITELIST_DB_CREDENTIALS").map_err(|src| {
        log::error!(
            "could not retrieve `PGSQL_WHITELIST_DB_CREDENTIALS` environment variable: {src}"
        );
        return Error::from(src);
    })?;

    log::trace!("connecting to database");

    todo!()
}
