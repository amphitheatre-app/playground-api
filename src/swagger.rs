// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{handlers, requests, responses};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::playbook::create,
        handlers::playbook::detail,
        handlers::playbook::update,
        handlers::playbook::delete,
    ),
    components(
        schemas(
            requests::playbook::CreatePlaybookRequest,
            requests::playbook::UpdatePlaybookRequest,
            responses::playbook::PlaybookResponse,
            amp_common::resource::PlaybookSpec,
            amp_common::resource::ActorSpec,
        )
    ),
    tags(
        (name = "Playbooks", description = "The Playbooks Service Handlers"),
    ),
)]
struct ApiDoc;

pub fn build() -> SwaggerUi {
    SwaggerUi::new("/swagger").url("/openapi.json", ApiDoc::openapi())
}
