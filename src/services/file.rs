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

use amp_common::sync::{EventKinds, Path, Synchronization};
use std::sync::Arc;
use tracing::debug;
use uuid::Uuid;

use amp_common::scm::content::Content;

use crate::context::Context;
use crate::errors::{ApiError, Result};
use crate::utils;

pub struct FileService;

impl FileService {
    /// Get a file content from the remote git repository.
    pub async fn get(ctx: Arc<Context>, id: Uuid, path: String) -> Result<Content> {
        let playbook = ctx.client.playbooks().get(&id.to_string()).map_err(ApiError::NotFoundPlaybook)?;
        let source = playbook.preface.repository.unwrap();

        ctx.github_client
            .contents()
            .find(&utils::repo(&source.repo)?, &path, &source.branch.unwrap_or_default())
            .map_err(|e| ApiError::NotFoundContent(e.to_string()))
    }

    /// Create a file to the workspace.
    pub async fn create(
        ctx: Arc<Context>,
        id: Uuid,
        path: String,
        content: String,
    ) -> Result<Content> {
        let data = content.into_bytes();
        let req = Synchronization {
            kind: EventKinds::Create,
            paths: vec![Path::File(path.clone())],
            attributes: None,
            payload: Some(data.clone()),
        };

        // Call sync() method to create file.
        FileService::sync(ctx, id, req)?;

        // We build a content object to return, because it's just a temporary file on the workspace.
        Ok(Content { path, data, sha: String::new(), blob_id: String::new() })
    }

    /// Update a file to the workspace.
    pub async fn update(
        _ctx: Arc<Context>,
        _id: Uuid,
        _path: String,
        _content: String,
    ) -> Result<Content> {
        // refer to create() method.
        todo!()
    }

    /// Delete a file from the workspace.
    pub async fn delete(_ctx: Arc<Context>, _id: Uuid, _path: String) -> Result<()> {
        // refer to create() method.
        todo!()
    }

    /// Copy a file to the destination path on the workspace.
    pub async fn copy(
        _ctx: Arc<Context>,
        _id: Uuid,
        _path: String,
        _destination: String,
    ) -> Result<Content> {
        // refer to create() method.
        todo!()
    }

    /// Move a file to the destination path on the workspace.
    pub async fn rename(
        _ctx: Arc<Context>,
        _id: Uuid,
        _path: String,
        _destination: String,
    ) -> Result<Content> {
        // refer to create() method.
        todo!()
    }

    /// Sync to the workspace.
    fn sync(ctx: Arc<Context>, id: Uuid, req: Synchronization) -> Result<u16> {
        let playbook = ctx.client.playbooks().get(&id.to_string()).map_err(ApiError::NotFoundPlaybook)?;

        debug!("update playbooks in {}...", id);

        if let Some(characters) = playbook.characters {
            let character = characters.first().unwrap();
            ctx.client.actors().sync(&id.to_string(), &character.meta.name, req).map_err(ApiError::FailedToSynchronize)
        } else {
            Err(ApiError::BadPlaybook("The playbook has no characters".to_string()))
        }
    }
}
