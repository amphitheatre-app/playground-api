// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use super::Result;
use crate::context::Context;
use crate::requests::playbook::{CreatePlaybookRequest, UpdatePlaybookRequest};
use crate::services::playbook::PlaybookService;

// The Playbooks Service Handlers.

/// Create a playbook in the current account.
#[utoipa::path(
    post, path = "/v1/playbooks",
    request_body(
        content = inline(CreatePlaybookRequest),
        description = "Create playbook request",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "Playbook created successfully", body = PlaybookResponse)
    ),
    tag = "Playbooks"
)]
pub async fn create(
    State(ctx): State<Arc<Context>>,
    Json(req): Json<CreatePlaybookRequest>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(PlaybookService::create(ctx, &req).await?)))
}

/// Returns a playbook detail.
#[utoipa::path(
    get, path = "/v1/playbooks/{id}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
    ),
    responses(
        (status = 200, description = "Playbook found successfully", body = PlaybookResponse),
        (status = 404, description = "Playbook not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Playbooks"
)]
pub async fn detail(Path(id): Path<Uuid>, State(ctx): State<Arc<Context>>) -> Result<impl IntoResponse> {
    Ok(Json(PlaybookService::get(ctx, id).await?))
}

/// Update a playbook.
#[utoipa::path(
    patch, path = "/v1/playbooks/{id}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
    ),
    request_body(
        content = inline(UpdatePlaybookRequest),
        description = "Update playbook request",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Playbook updated successfully", body = PlaybookResponse),
        (status = 404, description = "Playbook not found")
    ),
    tag = "Playbooks"
)]
pub async fn update(
    Path(id): Path<Uuid>,
    State(ctx): State<Arc<Context>>,
    Json(req): Json<UpdatePlaybookRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(PlaybookService::update(ctx, id, &req).await?))
}

/// Delete a playbook
#[utoipa::path(
    delete, path = "/v1/playbooks/{id}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
    ),
    responses(
        (status = 204, description = "Playbook deleted successfully"),
        (status = 404, description = "Playbook not found")
    ),
    tag = "Playbooks"
)]
pub async fn delete(Path(id): Path<Uuid>, State(ctx): State<Arc<Context>>) -> Result<impl IntoResponse> {
    PlaybookService::delete(ctx, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
