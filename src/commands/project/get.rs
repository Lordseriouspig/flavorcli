// Copyright (C) 2025 Lordseriouspig
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
use crate::helpers::print_project::print_project;
use crate::models::authdata::AuthData;
use crate::models::project::Project;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};

#[derive(Debug, Args)]
pub struct ProjectGet {
    // Defines get project command (level 3)
    /// The project ID to retrieve
    pub project_id: u64, // TODO: short flag (short output), long/resolve flag (resolve devlogs)

    /// Returns data as raw JSON
    #[clap(long)]
    pub json: bool,
}

impl ProjectGet {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!(
            "Executing project get command for project ID: {}",
            self.project_id
        );
        let auth: AuthData = get_key()?;
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Retrieving project...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let client = reqwest::Client::new();
        let url = format!(
            "https://flavortown.hackclub.com/api/v1/projects/{}",
            self.project_id
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
            spinner.finish_and_clear();
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
            spinner.finish_and_clear();
            info!("Retrieved project successfully.");
            if self.json {
                let project_json = res.text().await?;
                debug!("Returning raw JSON data");
                println!("{}", project_json);
            } else {
                let project: Project = res.json().await?;
                debug!("Successfully parsed project data");
                print_project(&project);
            }
            Ok(())
        }
    }
}
