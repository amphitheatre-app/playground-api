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
use uuid::Uuid;

use amp_common::scm::content::{Content, File};
use amp_common::scm::git::Tree;

use crate::context::Context;
use crate::errors::{ApiError, Result};
use crate::utils;
use crate::utils::unwrap_or_error;

pub struct FolderService;

impl FolderService {
    pub async fn get(ctx: Arc<Context>, id: Uuid, path: String) -> Result<Vec<File>, ApiError> {
        let playbook = ctx.client.playbooks().get(&id.to_string()).await.map_err(ApiError::NotFoundPlaybook)?;

        let source = unwrap_or_error(playbook.preface.repository, "The repository is none")?;
        let reference = unwrap_or_error(source.reference(), "The reference is none")?;

        ctx.github_client
            .contents()
            .list(&utils::repo(&source.repo)?, &path, &reference)
            .await
            .map_err(|e| ApiError::NotFoundContent(e.to_string()))
    }

    pub async fn tree(ctx: Arc<Context>, id: Uuid, recursive: Option<&String>) -> Result<Tree, ApiError> {
        let playbook = ctx.client.playbooks().get(&id.to_string()).await.map_err(ApiError::NotFoundPlaybook)?;

        let source = unwrap_or_error(playbook.preface.repository, "The repository is none")?;
        let reference = unwrap_or_error(source.reference(), "The reference is none")?;

        ctx.github_client
            .git()
            .get_tree(&utils::repo(&source.repo)?, &reference, Option::from(recursive.is_some()))
            .await
            .map_err(|e| ApiError::NotFoundFolder(e.to_string()))?
            .ok_or(ApiError::NotFoundFolder("The folder is none".to_string()))
    }

    pub async fn create(_ctx: Arc<Context>, _id: Uuid, _path: String) -> Result<Content> {
        todo!()
    }

    pub async fn delete(_ctx: Arc<Context>, _id: Uuid, _path: String) -> Result<()> {
        todo!()
    }

    pub async fn copy(_ctx: Arc<Context>, _id: Uuid, _path: String, _destination: String) -> Result<Content> {
        todo!()
    }

    pub async fn rename(_ctx: Arc<Context>, _id: Uuid, _path: String, _destination: String) -> Result<Content> {
        todo!()
    }
}
