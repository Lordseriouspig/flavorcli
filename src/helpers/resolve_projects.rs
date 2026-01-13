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

use crate::{helpers::get_key::get_key, models::authdata::AuthData, models::project::Project};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};

pub async fn resolve_projects(project_ids: &[u32]) -> Result<Vec<Project>> {
    let auth: AuthData = get_key()?;
    let spinner = ProgressBar::new_spinner();
    let mut resolved_projects = Vec::new();
    spinner.set_style(
        ProgressStyle::with_template("{spinner} {msg}")?
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.set_message("Resolving projects...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    let client = reqwest::Client::new();
    for project in project_ids {
        let url = format!(
            "https://flavortown.hackclub.com/api/v1/projects/{}",
            project
        );
        debug!("Sending GET request to {}", url);
        let res = client
            .get(&url)
            .header("Authorization", auth.token.clone())
            .header("X-Flavortown-Ext-333", "true")
            .send()
            .await?;
        debug!("Received response with status: {}", res.status());
        if !res.status().is_success() {
            anyhow::bail!(
                "Request failed with status: {}. {}",
                res.status(),
                match res.status().as_u16() {
                    401 => "Is your token correct?",
                    404 => "Is the project ID correct?",
                    _ => "Please try again later.",
                }
            );
        } else {
            resolved_projects.push(res.json::<Project>().await?);
            debug!("Successfully resolved project ID: {}", project);
        }
    }
    spinner.finish_and_clear();
    info!("Successfully resolved all projects");
    Ok(resolved_projects)
}
