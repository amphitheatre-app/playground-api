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

use std::convert::Infallible;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::{Stream, StreamExt};
use uuid::Uuid;

use crate::context::Context;
use crate::errors::{ApiError, Result};
use crate::services::LoggerService;

// The Logging Service Handlers.

/// get logs for a playbook.
#[utoipa::path(
    get, path = "/v1/playbooks/{id}/logs",
    params(
        ("id" = Uuid, description = "The id of playbook"),
    ),
    responses(
        (status = 200, description = "Playbook logs found successfully"),
        (status = 404, description = "Playbook not found")
    ),
    tag = "Logging"
)]
pub async fn logs(
    Path(id): Path<Uuid>,
    State(ctx): State<Arc<Context>>,
) -> Result<Sse<impl Stream<Item = axum::response::Result<Event, Infallible>>>, ApiError> {
    let event_source = LoggerService::logs(ctx, id).await?;

    let stream = event_source
        .map(|line| {
            if let Ok(reqwest_eventsource::Event::Message(message)) = line {
                Event::default().data(message.event)
            } else {
                Event::default()
            }
        })
        .map(Ok);

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
