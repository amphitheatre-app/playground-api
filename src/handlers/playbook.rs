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

use amp_common::resource::PlaybookSpec;

use crate::context::Context;
use crate::errors::Result;
use crate::requests::playbook::CreatePlaybookRequest;
use crate::services::PlaybookService;

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
        (status = 201, description = "Playbook created successfully", body = PlaybookSpec)
    ),
    tag = "Playbooks"
)]
pub async fn create(
    State(ctx): State<Arc<Context>>,
    Json(req): Json<CreatePlaybookRequest>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(PlaybookService::create(ctx, &req).await?)))
}

/// Delete a playbook
#[utoipa::path(
    delete, path = "/v1/playbooks/{id}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
    ),
    responses(
        (status = 204, description = "Playbook deleted successfully"),
        (status = 404, description = "Playbook not found"),
        (status = 500, description = "Failed to delete playbook")
    ),
    tag = "Playbooks"
)]
pub async fn delete(State(ctx): State<Arc<Context>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse> {
    PlaybookService::delete(ctx, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// start a playbook
#[utoipa::path(
    post, path = "/v1/playbooks/{id}/actions/start",
    params(
        ("id" = Uuid, description = "The id of playbook"),
    ),
    responses(
        (status = 204, description = "Playbook started successfully"),
        (status = 404, description = "Playbook not found"),
        (status = 500, description = "Failed to start playbook")
    ),
    tag = "Playbooks"
)]
pub async fn start(State(ctx): State<Arc<Context>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse> {
    PlaybookService::start(ctx, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
