// Copyright 2026 Lordseriouspig
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use reqwest::Client;
use std::collections::HashMap;
use log::debug;

pub struct Session {
    client: Client,
}

impl Session {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .pool_max_idle_per_host(10)
                .build()
                .unwrap(),
        }
    }
}

impl Session {
    pub async fn get(&self, url: &str, auth: String, params: Option<Vec<(&str, String)>>) -> reqwest::Result<reqwest::Response> {
        debug!(
            "Sending GET request to {}{}",
            url,
            if let Some(p) = &params {
                format!(
                    " with params: {}",
                    p.iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<String>>()
                        .join("&")
                )
            } else {
                String::new()
            }
        );
        let mut request = self.client.get(url).header("Authorization", &auth).header("X-Flavortown-Ext-333", "true");
        if let Some(p) = params {
            request = request.query(&p);
        }
        let res = request.send().await?;
        debug!("Received response with status: {}", res.status());
        Ok(res)
    }
}

impl Session {
    pub async fn post(&self, url: &str, body: HashMap<&str, std::string::String>, auth: String) -> reqwest::Result<reqwest::Response> {
        debug!(
            "Sending POST request to {}\n{}",
            url,
            body.iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join("\n")
        );
        let res = self.client
            .post(url)
            .header("Authorization", &auth)
            .header("X-Flavortown-Ext-333", "true")
            .json(&body)
            .send()
            .await?;
        debug!("Received response with status: {}", res.status());
        Ok(res)
    }
}

impl Session {
    pub async fn patch(&self, url: &str, body: HashMap<&str, std::string::String>, auth: String) -> reqwest::Result<reqwest::Response> {
        debug!("Sending PATCH request to {}", url);
        let res = self.client
            .patch(url)
            .header("Authorization", &auth)
            .header("X-Flavortown-Ext-333", "true")
            .json(&body)
            .send()
            .await?;
        debug!("Received response with status: {}", res.status());
        Ok(res)
    }
}