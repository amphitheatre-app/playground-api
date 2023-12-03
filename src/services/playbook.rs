// Copyright 2023 The Amphitheatre Authors.
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

use crate::context::Context;
use crate::requests::playbook::{CreatePlaybookRequest, UpdatePlaybookRequest};
use crate::responses::playbook::PlaybookResponse;
use crate::services::Result;

pub struct PlaybookService;

impl PlaybookService {
    pub async fn get(_ctx: Arc<Context>, _id: Uuid) -> Result<PlaybookResponse> {
        unimplemented!()
    }

    pub async fn delete(_ctx: Arc<Context>, _id: Uuid) -> Result<()> {
        unimplemented!()
    }

    pub async fn create(_ctx: Arc<Context>, _req: &CreatePlaybookRequest) -> Result<PlaybookResponse> {
        unimplemented!()
    }

    pub async fn update(_ctx: Arc<Context>, _id: Uuid, _req: &UpdatePlaybookRequest) -> Result<PlaybookResponse> {
        unimplemented!()
    }
}
