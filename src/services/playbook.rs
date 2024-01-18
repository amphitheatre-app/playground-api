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
use amp_common::sync::Synchronization;
use std::sync::Arc;
use tracing::info;
use url::Url;

use uuid::Uuid;

use crate::context::Context;
use crate::errors::ApiError;
use crate::requests::playbook::CreatePlaybookRequest;
use crate::responses::playbook::FilesResponse;
use crate::services::Result;

pub struct PlaybookService;

impl PlaybookService {
    pub async fn get(
        ctx: Arc<Context>,
        id: Uuid,
        reference: String,
        path: Option<String>,
        recursive: bool,
    ) -> Result<FilesResponse> {
        let mut files_response = FilesResponse { content: empty_content(), tree: Tree::default() };
        let playbook_result = ctx.client.playbooks().get(&id.to_string()).map_err(ApiError::NotFoundPlaybook).ok();
        match playbook_result {
            Some(playbook) => {
                let repository = playbook.preface.manifest.unwrap_or_default().meta.repository;
                if let Ok(url) = Url::parse(repository.as_str()) {
                    let repo = &url.path()[1..];
                    match path.is_some() {
                        true => {
                            let content = ctx
                                .github_client
                                .contents()
                                .find(repo, path.unwrap().as_str(), reference.as_str())
                                .ok();
                            files_response.content = content.unwrap_or(empty_content());
                        }
                        false => {
                            let tree = ctx
                                .github_client
                                .git()
                                .git_trees(repo, reference.as_str(), Option::from(recursive))
                                .ok()
                                .unwrap_or_default();
                            files_response.tree = tree.unwrap_or_default();
                        }
                    }
                }
            }
            None => {
                info!("Not found playbooks in {}...", id);
            }
        }
        Ok(files_response)
    }

    pub async fn delete(ctx: Arc<Context>, id: Uuid) -> Result<u16> {
        info!("delete playbooks in {}...", id);
        ctx.client.playbooks().delete(&id.to_string()).map_err(ApiError::NotFoundPlaybook)
    }

    pub async fn create(ctx: Arc<Context>, req: &CreatePlaybookRequest) -> Result<PlaybookSpec> {
        let payload = PlaybookPayload {
            title: req.title.clone(),
            description: req.description.clone().unwrap_or_default(),
            preface: req.preface.clone(),
        };
        ctx.client.playbooks().create(payload).map_err(ApiError::FailedToCreatePlaybook)
    }

    pub async fn update(ctx: Arc<Context>, id: Uuid, req: Synchronization) -> Result<u16> {
        let playbook_result = ctx.client.playbooks().get(&id.to_string()).map_err(ApiError::NotFoundPlaybook).ok();
        return match playbook_result {
            Some(playbook) => {
                info!("update playbooks in {}...", id);
                let uuid_str: &str = &id.to_string();
                let name = playbook.preface.name.as_str();
                ctx.client.actors().sync(uuid_str, name, req).map_err(ApiError::NotFoundPlaybook)
            }
            None => {
                info!("Not found playbooks in {}...", id);
                Ok(0)
            }
        };
    }

    pub async fn start(ctx: Arc<Context>, id: Uuid) -> Result<u16> {
        info!("Start playbooks in {}...", id);
        ctx.client.playbooks().start(&id.to_string()).map_err(ApiError::NotFoundPlaybook)
    }

    pub async fn logs(_ctx: Arc<Context>, _id: Uuid) {
        unreachable!()
    }
}

fn empty_content() -> Content {
    Content { path: String::new(), data: Vec::new(), sha: String::new(), blob_id: String::new() }
}
