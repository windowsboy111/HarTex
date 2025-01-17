/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

//! # Guild Repository

use std::borrow::Cow;
use std::pin::Pin;

use hartex_database_queries::discord_frontend::queries::cached_guild_select_by_id::cached_guild_select_by_id;
use hartex_database_queries::discord_frontend::queries::cached_guild_upsert::cached_guild_upsert;
use hartex_discord_core::discord::model::guild::DefaultMessageNotificationLevel;
use hartex_discord_core::discord::model::guild::ExplicitContentFilter;
use hartex_discord_core::discord::model::guild::MfaLevel;
use hartex_discord_core::discord::model::guild::PremiumTier;
use hartex_discord_core::discord::model::guild::VerificationLevel;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::guild::GuildEntity;
use hartex_discord_utils::DATABASE_POOL;
use tokio_postgres::GenericClient;

/// Repository for guild entities.
pub struct CachedGuildRepository;

impl Repository<GuildEntity> for CachedGuildRepository {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    async fn get(&self, id: <GuildEntity as Entity>::Id) -> CacheResult<GuildEntity> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        let data = cached_guild_select_by_id()
            .bind(client, &id.to_string())
            .one()
            .await?;

        Ok(GuildEntity::from(data))
    }

    #[allow(clippy::cast_possible_wrap)]
    async fn upsert(&self, entity: GuildEntity) -> CacheResult<()> {
        let pinned = Pin::static_ref(&DATABASE_POOL).await;
        let pooled = pinned.get().await?;
        let client = pooled.client();

        cached_guild_upsert()
            .bind(
                client,
                &i16::from(<DefaultMessageNotificationLevel as Into<u8>>::into(
                    entity.default_message_notifications,
                )),
                &i16::from(<ExplicitContentFilter as Into<u8>>::into(
                    entity.explicit_content_filter,
                )),
                &entity
                    .features
                    .iter()
                    .map(|feature| feature.clone().into())
                    .collect::<Vec<Cow<'static, str>>>(),
                &entity.icon.map(|hash| hash.to_string()),
                &entity.large,
                &entity.name,
                &entity.owner_id.to_string(),
                &entity.id.to_string(),
                &i16::from(<MfaLevel as Into<u8>>::into(entity.mfa_level)),
                &entity.premium_subscription_count.map(|id| id as i64),
                &i16::from(<PremiumTier as Into<u8>>::into(entity.premium_tier)),
                &i16::from(<VerificationLevel as Into<u8>>::into(
                    entity.verification_level,
                )),
            )
            .await?;

        Ok(())
    }
}
