// Copyright (C) 2026 Lordseriouspig
//
// This file is part of flavorcli.
//
// flavorcli is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// flavorcli is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with flavorcli.  If not, see <https://www.gnu.org/licenses/>.

use crate::helpers::get_key::get_key;
use crate::models::authdata::AuthData;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;

#[derive(Debug, Args)]
pub struct ProjectDevlogDelete {
    // Defines delete devlog command (level 3)
    /// The id of the project to delete a devlog from
    pub project_id: u32,

    /// The id of the devlog to delete
    pub devlog_id: u32,
}

impl ProjectDevlogDelete {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!("Executing devlog delete command.");
        let auth: AuthData = get_key()?;
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Deleting devlog...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let client = reqwest::Client::new();
        let url = format!(
            "https://flavortown.hackclub.com/api/v1/projects/{}/devlogs/{}",
            self.project_id, self.devlog_id
        );

        debug!("Sending DELETE request to {}", url);
        let res = client
            .delete(&url)
            .header("Authorization", auth.token.clone())
            .header("X-Flavortown-Ext-333", "true")
            .send()
            .await?;
        debug!("Received response with status: {}", res.status());
        if !res.status().is_success() {
            spinner.finish_and_clear();
            anyhow::bail!(
                "Request failed with status: {}. {}",
                res.status(),
                match res.status().as_u16() {
                    401 => "Is your token correct?".to_string(),
                    404 => "Is the Project ID and Devlog ID correct?".to_string(),
                    422 => {
                        let errors: serde_json::Value = res.json().await.unwrap_or_default();
                        let messages = errors
                            .get("errors")
                            .and_then(|e| e.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str())
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            })
                            .unwrap_or_else(|| errors.to_string());
                        format!("You did something wrong, didn't you?\n{}", messages)
                    }
                    _ => "Please try again later.".to_string(),
                }
            );
        } else {
            spinner.finish_and_clear();
            println!("Deleted devlog successfully.");
            Ok(())
        }
    }
}
