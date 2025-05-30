// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use amp_common::http::HTTPError;
use amp_common::scm::errors::SCMError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use thiserror::Error;
use tracing::error;

pub type Result<T, E = ApiError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not Found")]
    NotFound,

    #[error("Not Found Playbook: {0}")]
    NotFoundPlaybook(HTTPError),

    #[error("Failed to create playbook: {0}")]
    FailedToCreatePlaybook(HTTPError),

    #[error("Failed to delete playbook: {0}")]
    FailedToDeletePlaybook(HTTPError),

    #[error("Failed to start playbook: {0}")]
    FailedToStartPlaybook(HTTPError),

    #[error("Not Found Content: {0}")]
    NotFoundContent(String),

    #[error("InvalidRepoAddress: {0}")]
    InvalidRepoAddress(#[source] url::ParseError),

    #[error("Not Found Folder: {0}")]
    NotFoundFolder(String),

    #[error("Failed to synchronize: {0}")]
    FailedToSynchronize(HTTPError),

    #[error("Bad Playbook: {0}")]
    BadPlaybook(String),

    #[error("Not Found Repo: {0}")]
    NotFoundRepo(SCMError),

    #[error("Bad Playbook Request: {0}")]
    BadPlaybookRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::NotFoundPlaybook(e) => (StatusCode::NOT_FOUND, e.to_string()),
            Self::FailedToCreatePlaybook(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Self::FailedToDeletePlaybook(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::FailedToStartPlaybook(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::NotFoundContent(e) => (StatusCode::NOT_FOUND, e.to_string()),
            Self::InvalidRepoAddress(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::NotFoundFolder(e) => (StatusCode::NOT_FOUND, e.to_string()),
            Self::FailedToSynchronize(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::BadPlaybook(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Self::NotFoundRepo(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::BadPlaybookRequest(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };

        error!("{} - {}", status, message);
        (status, Json(json!({ "message": message }))).into_response()
    }
}
