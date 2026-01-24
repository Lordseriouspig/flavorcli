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
use crate::helpers::print_project_table::print_project_table;
use crate::models::authdata::AuthData;
use crate::models::project_vec::ProjectVec;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct ProjectList {
    // Defines list projects command (level 3)
    /// Page number for pagination. Defaults to 1.
    #[clap(long, short)]
    pub page: Option<u32>,

    /// Query string to filter projects
    #[clap(long, alias = "search")]
    pub query: Option<String>,

    /// Returns data as raw JSON
    #[clap(long)]
    pub json: bool,

    /// Fields to output in the table (advanced)
    #[clap(
        long,
        value_enum,
        conflicts_with = "json",
        value_delimiter = ',',
        default_value = "id,title,description,updated-at"
    )]
    pub fields: Vec<ProjectFields>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectFields {
    Id,
    Title,
    Description,
    ShipStatus,
    RepoUrl,
    DemoUrl,
    ReadmeUrl,
    CreatedAt,
    UpdatedAt,
    DevlogIds,
}

impl ProjectList {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!(
            "Executing project list command (page: {:?}, query: {:?})",
            self.page, self.query
        );
        let auth: AuthData = get_key()?;
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Retrieving projects...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let client = reqwest::Client::new();
        let params = {
            let mut p = vec![];
            if let Some(page) = self.page {
                p.push(("page", page.to_string()));
            }
            if let Some(query) = &self.query {
                p.push(("query", query.clone()));
            }
            p
        };
        debug!(
            "Sending GET request to https://flavortown.hackclub.com/api/v1/projects with params: {:?}",
            params
        );
        let res = client
            .get("https://flavortown.hackclub.com/api/v1/projects")
            .query(&params)
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
                    404 => "Could not find what you were lookin' for!",
                    _ => "Please try again later.",
                }
            );
        } else {
            spinner.finish_and_clear();
            info!("Retrieved projects successfully.");
            if self.json {
                let projects_json = res.text().await?;
                debug!("Returning raw JSON data");
                println!("{}", projects_json);
            } else {
                let projects: ProjectVec = res.json().await?;
                debug!("Successfully parsed {} projects", projects.projects.len());
                if let Some(query) = &self.query {
                    println!(
                        "{}{}{}",
                        "Search result(s) for query '".bold().cyan(),
                        query.italic().bold().yellow(),
                        "'".bold().cyan()
                    );
                }
                print_project_table(
                    &projects.projects,
                    &projects.pagination,
                    self.fields.clone(),
                );
            }
        }
        Ok(())
    }
}
