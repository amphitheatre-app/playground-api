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

use crate::config::Config;
use amp_client::client::Client;
use amp_common::scm::client::Client as ScmClient;
use amp_common::scm::driver::github;
use std::sync::Arc;

/// The core type through which handler functions can access common API state.
///
/// This can be accessed by adding a parameter `Extension<Context>` to a handler
///  function's  parameters.
///
/// It may not be a bad idea if you need your API to be more modular (turn routes
/// on and off, and disable any unused extension objects) but it's really up to a
/// judgement call.
#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub client: Arc<Client>,
    pub github_client: Arc<ScmClient>,
}

impl Context {
    pub async fn new(config: Config) -> anyhow::Result<Context> {
        let client = Arc::new(Client::new(&config.amp_server, config.auth_token.clone()));
        let github_client = Arc::new(ScmClient::new(github::default()));
        Ok(Context { config, client, github_client })
    }
}
