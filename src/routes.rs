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

use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::context::Context;
use crate::handlers;

pub fn build() -> Router<Arc<Context>> {
    Router::new()
        // playbooks
        .route("/v1/playbooks", post(handlers::playbook::create))
        .route("/v1/playbooks/:id/files/:reference/:path", get(handlers::playbook::detail))
        .route("/v1/playbooks/:id", put(handlers::playbook::update))
        .route("/v1/playbooks/:id", delete(handlers::playbook::delete))
        .route("/v1/playbooks/:id/actions/start", get(handlers::playbook::start))
}
