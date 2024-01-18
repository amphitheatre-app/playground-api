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
use crate::handlers::{file, folder, logger, playbook};

pub fn build() -> Router<Arc<Context>> {
    Router::new()
        // playbooks
        .route("/v1/playbooks", post(playbook::create))
        .route("/v1/playbooks/:id", delete(playbook::delete))
        .route("/v1/playbooks/:id/actions/start", get(playbook::start))
        //
        // logging
        .route("/v1/playbooks/:id/logs", get(logger::logs))
        //
        // files
        .route("/v1/playbooks/:id/files/:reference/:path", get(file::get))
        .route("/v1/playbooks/:id/files/:reference/:path", post(file::create))
        .route("/v1/playbooks/:id/files/:reference/:path", put(file::update))
        .route("/v1/playbooks/:id/files/:reference/:path", delete(file::delete))
        .route("/v1/playbooks/:id/files/:reference/:path/actions/copy", post(file::copy))
        .route("/v1/playbooks/:id/files/:reference/:path/actions/move", post(file::rename))
        //
        // folders
        .route("/v1/playbooks/:id/folders/:reference/:path", get(folder::get))
        .route("/v1/playbooks/:id/folders/:reference/:path", post(folder::create))
        .route("/v1/playbooks/:id/folders/:reference/:path", delete(folder::delete))
        .route("/v1/playbooks/:id/folders/:reference/:path/actions/copy", post(folder::copy))
        .route("/v1/playbooks/:id/folders/:reference/:path/actions/move", post(folder::rename))
}
