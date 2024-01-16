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

use amp_client::playbooks::PlaybookPayload;
use amp_common::resource::PlaybookSpec;
use amp_common::scm::content::Content;
use amp_common::scm::git::Tree;
use std::sync::Arc;

use uuid::Uuid;

use crate::context::Context;
use crate::errors::ApiError;
use crate::requests::playbook::{CreatePlaybookRequest, UpdatePlaybookRequest};
use crate::services::Result;

pub struct PlaybookService;

impl PlaybookService {
    pub async fn get(ctx: Arc<Context>, id: Uuid, reference: String, path: String) -> Result<Content> {
        let result = ctx.client.playbooks().get(&id.to_string()).map_err(ApiError::NotFoundPlaybook);
        let spec = result.unwrap_or_default();
        let repo = spec.preface.repository.unwrap_or_default().repo;
        let content = ctx.github_client.contents().find(repo.as_str(), path.as_str(), reference.as_str()).ok();
        Ok(content.unwrap())
    }

    pub async fn trees(ctx: Arc<Context>, id: Uuid, reference: String, recursive: Option<bool>) -> Result<Tree> {
        let result = ctx.client.playbooks().get(&id.to_string()).map_err(ApiError::NotFoundPlaybook);
        let spec = result.unwrap_or_default();
        let repo = spec.preface.repository.unwrap_or_default().repo;
        let tree =
            ctx.github_client.git().git_trees(repo.as_str(), reference.as_str(), recursive).ok().unwrap_or_default();
        Ok(tree.unwrap_or_default())
    }

    pub async fn delete(ctx: Arc<Context>, id: Uuid) -> Result<u16> {
        ctx.client.playbooks().delete(&id.to_string()).map_err(ApiError::NotFoundPlaybook)
    }

    pub async fn create(ctx: Arc<Context>, req: &CreatePlaybookRequest) -> Result<PlaybookSpec> {
        let playbook_payload = PlaybookPayload {
            title: req.title.clone(),
            description: req.description.clone().unwrap_or_default(),
            preface: req.preface.clone(),
        };
        ctx.client.playbooks().create(playbook_payload).map_err(ApiError::NotFoundPlaybook)
    }

    pub async fn update(_ctx: Arc<Context>, _id: Uuid, _req: &UpdatePlaybookRequest) -> Result<PlaybookSpec> {
        unimplemented!()
    }

    pub async fn start(ctx: Arc<Context>, id: Uuid) -> Result<u16> {
        ctx.client.playbooks().start(&id.to_string()).map_err(ApiError::NotFoundPlaybook)
    }
}
