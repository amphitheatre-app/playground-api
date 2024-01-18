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

use crate::{handlers, requests};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::playbook::create,
        handlers::playbook::detail,
        handlers::playbook::update,
        handlers::playbook::delete,
        handlers::playbook::start,

        handlers::logger::logs,
    ),
    components(
        schemas(
            requests::playbook::CreatePlaybookRequest,
            requests::playbook::UpdatePlaybookRequest,

            amp_common::resource::ActorSpec,
            amp_common::resource::CharacterSpec,
            amp_common::resource::Partner,
            amp_common::resource::PlaybookSpec,
            amp_common::resource::Preface,

            amp_common::schema::Build,
            amp_common::schema::BuildpacksConfig,
            amp_common::schema::Deploy,
            amp_common::schema::DockerfileConfig,
            amp_common::schema::GitReference,
            amp_common::schema::LocalPartner,
            amp_common::schema::Metadata,
            amp_common::schema::Port,
            amp_common::schema::RegisteredPartner,
            amp_common::schema::Service,

            amp_common::scm::content::Content,
            amp_common::scm::git::Tree,
            amp_common::scm::git::TreeEntry,

            amp_common::sync::Synchronization,
        )
    ),
    tags(
        (name = "Playbooks", description = "The Playbooks Service Handlers"),
        (name = "Logger", description = "The Logger Service Handlers"),
    ),
)]
struct ApiDoc;

pub fn build() -> SwaggerUi {
    SwaggerUi::new("/swagger").url("/openapi.json", ApiDoc::openapi())
}
