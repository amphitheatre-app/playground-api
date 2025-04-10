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

use reqwest_eventsource::EventSource;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::Context;
use crate::errors::ApiError;

pub struct LoggerService;

impl LoggerService {
    pub async fn logs(ctx: Arc<Context>, id: Uuid) -> Result<EventSource, ApiError> {
        let playbook = ctx.client.playbooks().get(&id.to_string()).await.map_err(ApiError::NotFoundPlaybook)?;

        if let Some(characters) = playbook.characters {
            let character = characters.first().unwrap();
            Ok(ctx.client.actors().logs(&id.to_string(), &character.meta.name))
        } else {
            Err(ApiError::BadPlaybook("The playbook has no characters".to_string()))
        }
    }
}
