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

use std::future::Future;
use std::pin::Pin;

use chrono::DateTime as ChronoDateTime;
use chrono::Utc;

use hartex_bors_core::models::BorsBuild;
use hartex_bors_core::models::BorsBuildStatus;
use hartex_bors_core::models::BorsPullRequest;
use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::DatabaseClient;
use hartex_eyre::eyre::Report;
use sea_orm::prelude::DateTime;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

use crate::entity;

/// A SeaORM database client.
pub struct SeaORMDatabaseClient {
    connection: DatabaseConnection,
}

impl SeaORMDatabaseClient {
    /// Construct a new database client.
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl DatabaseClient for SeaORMDatabaseClient {
    fn get_or_create_pull_request<'a>(
        &'a self,
        name: &'a GithubRepositoryName,
        pr_number: u64,
    ) -> Pin<Box<dyn Future<Output = hartex_eyre::Result<BorsPullRequest>> + '_>> {
        Box::pin(async move {
            let pr = entity::pull_request::ActiveModel {
                repository: Set(format!("{name}")),
                number: Default::default(),
                ..Default::default()
            };

            match entity::pull_request::Entity::insert(pr)
                .on_conflict(OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&self.connection)
                .await
            {
                Ok(_) => {}
                Err(DbErr::RecordNotInserted) => {
                    // the record is already in the database
                }
                Err(error) => return Err(error.into()),
            }

            let (pr, build) = entity::pull_request::Entity::find()
                .filter(
                    entity::pull_request::Column::Repository
                        .eq(format!("{name}"))
                        .and(entity::pull_request::Column::Number.eq(pr_number)),
                )
                .find_also_related(entity::build::Entity)
                .one(&self.connection)
                .await?
                .ok_or_else(|| Report::msg("cannot execute query"))?;

            Ok(pr_from_database(pr, build))
        })
    }
}

fn build_from_database(model: entity::build::Model) -> BorsBuild {
    BorsBuild {
        id: model.id,
        repository: model.repository,
        branch: model.branch,
        commit_hash: model.commit_hash,
        status: build_status_from_database(model.status),
        created_at: datetime_from_database(model.created_at),
    }
}

fn build_status_from_database(status: String) -> BorsBuildStatus {
    match status.as_str() {
        "pending" => BorsBuildStatus::Pending,
        "success" => BorsBuildStatus::Success,
        "failure" => BorsBuildStatus::Failure,
        "cancelled" => BorsBuildStatus::Cancelled,
        _ => BorsBuildStatus::Pending,
    }
}

fn datetime_from_database(datetime: DateTime) -> DateTimeUtc {
    ChronoDateTime::from_utc(datetime, Utc)
}

fn pr_from_database(
    pr: entity::pull_request::Model,
    build: Option<entity::build::Model>,
) -> BorsPullRequest {
    BorsPullRequest {
        id: pr.id,
        repository: pr.repository,
        number: pr.number as u64,
        try_build: build.map(build_from_database),
        created_at: datetime_from_database(pr.created_at),
    }
}