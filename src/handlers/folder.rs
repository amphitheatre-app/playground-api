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

use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::context::Context;
use crate::errors::Result;
use crate::requests::file::DestinationRequest;
use crate::services::FolderService;

// The Folders Service Handlers.

/// Returns a folder's tree.
#[utoipa::path(
    get, path = "/v1/playbooks/{id}/folders/{reference}/{path}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("reference" = String, description = "The name of the commit/branch/tag."),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    responses(
        (status = 200, description = "The folder tree", body = Tree),
        (status = 404, description = "Playbook not found"),
        (status = 404, description = "Folder not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Folders"
)]
pub async fn get(
    State(ctx): State<Arc<Context>>,
    Path((id, reference, path)): Path<(Uuid, String, Option<String>)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    Ok(Json(FolderService::get(ctx, id, reference, path, params.get("recursive")).await?))
}

/// Create a folder
#[utoipa::path(
    post, path = "/v1/playbooks/{id}/folders/{reference}/{path}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("reference" = String, description = "The name of the commit/branch/tag."),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    responses(
        (status = 201, description = "The folder created successfully", body = Tree),
        (status = 404, description = "Playbook not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Folders"
)]
pub async fn create(
    State(ctx): State<Arc<Context>>,

    Path(id): Path<Uuid>,
    Path(reference): Path<String>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(FolderService::create(ctx, id, reference, path).await?)))
}

/// Delete a folder
#[utoipa::path(
    delete, path = "/v1/playbooks/{id}/folders/{reference}/{path}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("reference" = String, description = "The name of the commit/branch/tag."),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    responses(
        (status = 204, description = "The folder deleted successfully"),
        (status = 404, description = "Playbook not found"),
        (status = 404, description = "Folder not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Folders"
)]
pub async fn delete(
    State(ctx): State<Arc<Context>>,

    Path(id): Path<Uuid>,
    Path(reference): Path<String>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse> {
    FolderService::delete(ctx, id, reference, path).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Copy a folder
#[utoipa::path(
    post, path = "/v1/playbooks/{id}/folders/{reference}/{path}/actions/copy",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("reference" = String, description = "The name of the commit/branch/tag. Default: default branch."),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    request_body(
        content = inline(DestinationRequest),
        description = "The destination request",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The folder copied successfully", body = Content),
        (status = 404, description = "Playbook not found"),
        (status = 404, description = "Folder not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Folders"
)]
pub async fn copy(
    State(ctx): State<Arc<Context>>,

    Path(id): Path<Uuid>,
    Path(reference): Path<String>,
    Path(path): Path<String>,

    Json(req): Json<DestinationRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(FolderService::copy(ctx, id, reference, path, req.destination).await?))
}

/// Move a folder
#[utoipa::path(
    post, path = "/v1/playbooks/{id}/folders/{reference}/{path}/actions/move",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("reference" = String, description = "The name of the commit/branch/tag. Default: default branch."),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    request_body(
        content = inline(DestinationRequest),
        description = "The destination request",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The folder moved successfully", body = Content),
        (status = 404, description = "Playbook not found"),
        (status = 404, description = "Folder not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Folders"
)]
pub async fn rename(
    State(ctx): State<Arc<Context>>,

    Path(id): Path<Uuid>,
    Path(reference): Path<String>,
    Path(path): Path<String>,

    Json(req): Json<DestinationRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(FolderService::rename(ctx, id, reference, path, req.destination).await?))
}
