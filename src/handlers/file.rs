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

use crate::context::Context;
use crate::errors::Result;
use crate::requests::file::{DestinationRequest, FileRequest};
use crate::services::FileService;

// The Files Service Handlers.

/// Returns a file's content.
#[utoipa::path(
    get, path = "/v1/playbooks/{id}/files/{path}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    responses(
        (status = 200, description = "The file content", body = Content),
        (status = 404, description = "Playbook not found"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Files"
)]
pub async fn get(
    State(ctx): State<Arc<Context>>,
    Path((id, path)): Path<(Uuid, String)>,
) -> Result<impl IntoResponse> {
    Ok(Json(FileService::get(ctx, id, path).await?))
}

/// Create a file
#[utoipa::path(
    post, path = "/v1/playbooks/{id}/files/{path}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    request_body(
        content = inline(FileRequest),
        description = "Create file request",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "The file created successfully", body = Content),
        (status = 404, description = "Playbook not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Files"
)]
pub async fn create(
    State(ctx): State<Arc<Context>>,

    Path(id): Path<Uuid>,
    Path(path): Path<String>,

    Json(req): Json<FileRequest>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(FileService::create(ctx, id, path, req.content).await?)))
}

/// Update a file
#[utoipa::path(
    put, path = "/v1/playbooks/{id}/files/{path}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    request_body(
        content = inline(FileRequest),
        description = "Update file request",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The file updated successfully", body = Content),
        (status = 404, description = "Playbook not found"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Files"
)]
pub async fn update(
    State(ctx): State<Arc<Context>>,

    Path(id): Path<Uuid>,
    Path(path): Path<String>,

    Json(req): Json<FileRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(FileService::update(ctx, id, path, req.content).await?))
}

/// Delete a file
#[utoipa::path(
    delete, path = "/v1/playbooks/{id}/files/{path}",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    responses(
        (status = 204, description = "The file deleted successfully"),
        (status = 404, description = "Playbook not found"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Files"
)]
pub async fn delete(
    State(ctx): State<Arc<Context>>,

    Path(id): Path<Uuid>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse> {
    FileService::delete(ctx, id, path).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Copy a file
#[utoipa::path(
    post, path = "/v1/playbooks/{id}/files/{path}/actions/copy",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    request_body(
        content = inline(DestinationRequest),
        description = "The destination request",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The file copied successfully", body = Content),
        (status = 404, description = "Playbook not found"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Files"
)]
pub async fn copy(
    State(ctx): State<Arc<Context>>,

    Path(id): Path<Uuid>,
    Path(path): Path<String>,

    Json(req): Json<DestinationRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(FileService::copy(ctx, id, path, req.destination).await?))
}

/// Move a file
#[utoipa::path(
    post, path = "/v1/playbooks/{id}/files/{path}/actions/move",
    params(
        ("id" = Uuid, description = "The id of playbook"),
        ("path" = String, description = "The file path relative to the root of the repository."),
    ),
    request_body(
        content = inline(DestinationRequest),
        description = "The destination request",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "The file moved successfully", body = Content),
        (status = 404, description = "Playbook not found"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Files"
)]
pub async fn rename(
    State(ctx): State<Arc<Context>>,

    Path(id): Path<Uuid>,
    Path(path): Path<String>,

    Json(req): Json<DestinationRequest>,
) -> Result<impl IntoResponse> {
    Ok(Json(FileService::rename(ctx, id, path, req.destination).await?))
}
