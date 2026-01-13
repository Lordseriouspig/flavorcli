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
use crate::helpers::print_project::print_project;
use crate::models::authdata::AuthData;
use crate::models::project::Project;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use std::collections::HashMap;

#[derive(Debug, Args)]
pub struct ProjectCreate {
    // Defines create project command (level 3)
    /// The title of the new project
    #[clap(long)]
    pub title: String,

    /// The description of the new project
    #[clap(long)]
    pub description: String,

    /// A link to the project's repository
    #[clap(long)]
    pub repo_url: Option<String>,

    /// A link to the project's demo
    #[clap(long)]
    pub demo_url: Option<String>,

    /// A link to the project's readme
    #[clap(long)]
    pub readme_url: Option<String>,

    /// Returns data as raw JSON
    #[clap(long)]
    pub json: bool,
}

impl ProjectCreate {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!("Executing project create command.");
        let auth: AuthData = get_key()?;
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Creating project...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let client = reqwest::Client::new();
        let url = "https://flavortown.hackclub.com/api/v1/projects".to_string();
        let mut body = HashMap::new();

        body.insert("title", self.title.clone());
        body.insert("description", self.description.clone());

        if let Some(repo_url) = &self.repo_url {
            body.insert("repo_url", repo_url.clone());
        }
        if let Some(demo_url) = &self.demo_url {
            body.insert("demo_url", demo_url.clone());
        }
        if let Some(readme_url) = &self.readme_url {
            body.insert("readme_url", readme_url.clone());
        }

        debug!(
            "Sending POST request to {}\n{}",
            url,
            body.iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join("\n")
        );
        let res = client
            .post(&url)
            .header("Authorization", auth.token.clone())
            .header("X-Flavortown-Ext-333", "true")
            .form(&body)
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
                    404 => "Could not find what you're looking for!".to_string(),
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
            info!("Created project successfully.");
            if self.json {
                let project_json = res.text().await?;
                debug!("Returning raw JSON data");
                println!("{}", project_json);
            } else {
                let project: Project = res.json().await?;
                debug!("Successfully parsed project data");
                print_project(&project, false).await;
            }
            Ok(())
        }
    }
}
