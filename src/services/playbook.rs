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
use std::sync::Arc;
use tracing::{error, info};

use uuid::Uuid;

use crate::context::Context;
use crate::errors::ApiError;
use crate::errors::Result;
use crate::requests::playbook::CreatePlaybookRequest;

pub struct PlaybookService;

impl PlaybookService {
    pub async fn create(ctx: Arc<Context>, req: &CreatePlaybookRequest) -> Result<PlaybookSpec> {
        let payload = PlaybookPayload {
            title: req.title.clone(),
            description: req.description.clone().unwrap_or_default(),
            preface: req.preface.clone(),
        };
        ctx.client.playbooks().create(payload).map_err(ApiError::FailedToCreatePlaybook)
    }

    pub async fn delete(ctx: Arc<Context>, id: Uuid) -> Result<u16> {
        let playbooks = ctx.client.playbooks();
        match playbooks.get(&id.to_string()) {
            Ok(_) => {
                info!("delete playbooks in {}...", id);
                playbooks.delete(&id.to_string()).map_err(ApiError::FailedToDeletePlaybook)
            }
            Err(e) => {
                error!("Not found playbooks in {}, error: {}", id, e.to_string());
                Err(ApiError::NotFoundPlaybook(e))
            }
        }
    }

    pub async fn start(ctx: Arc<Context>, id: Uuid) -> Result<u16> {
        let playbooks = ctx.client.playbooks();
        match playbooks.get(&id.to_string()) {
            Ok(_) => {
                info!("Start playbooks in {}...", id);
                playbooks.start(&id.to_string()).map_err(ApiError::FailedToStartPlaybook)
            }
            Err(e) => {
                error!("Not found playbooks in {}, error: {}", id, e.to_string());
                Err(ApiError::NotFoundPlaybook(e))
            }
        }
    }
}
