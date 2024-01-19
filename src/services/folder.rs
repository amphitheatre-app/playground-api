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

use amp_common::scm::git::Tree;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use amp_common::scm::content::Content;

use crate::context::Context;
use crate::errors::{ApiError, Result};
use crate::utils;

pub struct FolderService;

impl FolderService {
    // @FIXME: pass path to get_trees() method.
    // @TODO: add recursive option argument to current method.
    pub async fn get(
        ctx: Arc<Context>,
        id: Uuid,
        _reference: String,
        path: Option<String>,
        recursive: HashMap<String, String>,
    ) -> Result<Tree> {
        let playbook = ctx.client.playbooks().get(&id.to_string()).map_err(ApiError::NotFoundPlaybook)?;
        let source = playbook.preface.repository.unwrap();
        let recursive = recursive.get("recursive").is_some();

        let content = ctx
            .github_client
            .contents()
            .find(&utils::repo(&source.repo)?, &path.unwrap_or_default(), &source.branch.unwrap_or_default())
            .map_err(|e| ApiError::NotFoundContent(e.to_string()))?;

        ctx.github_client
            .git()
            .get_tree(&utils::repo(&source.repo)?, &content.sha, Some(recursive))
            .map_err(|e| ApiError::NotFoundFolder(e.to_string()))?
            .ok_or(ApiError::NotFoundFolder("The folder is none".to_string()))
    }

    pub async fn create(_ctx: Arc<Context>, _id: Uuid, _reference: String, _path: String) -> Result<Content> {
        todo!()
    }

    pub async fn delete(_ctx: Arc<Context>, _id: Uuid, _reference: String, _path: String) -> Result<()> {
        todo!()
    }

    pub async fn copy(
        _ctx: Arc<Context>,
        _id: Uuid,
        _reference: String,
        _path: String,
        _destination: String,
    ) -> Result<Content> {
        todo!()
    }

    pub async fn rename(
        _ctx: Arc<Context>,
        _id: Uuid,
        _reference: String,
        _path: String,
        _destination: String,
    ) -> Result<Content> {
        todo!()
    }
}
