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
use crate::helpers::print_devlog_table::print_devlog_table;
use crate::models::authdata::AuthData;
use crate::models::devlog_vec::DevlogVec;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct ProjectDevlogList {
    // Defines list devlogs command (level 4)
    /// The project ID to list devlogs for. Will list ALL devlogs if not provided.
    pub project_id: Option<u64>,

    /// Page number for pagination. Defaults to 1.
    #[clap(long, short)]
    pub page: Option<u32>,
}

impl ProjectDevlogList {
    pub async fn execute(&self) -> anyhow::Result<()> {
        let auth: AuthData = get_key()?;
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Retrieving devlogs...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let client = reqwest::Client::new();
        let params = {
            let mut p = vec![];
            if let Some(page) = self.page {
                p.push(("page", page.to_string()));
            }
            p
        };
        let res = client
            .get(&format!(
                "https://flavortown.hackclub.com{}",
                if let Some(project_id) = self.project_id {
                    format!("/api/v1/projects/{}/devlogs", project_id)
                } else {
                    format!("/api/v1/devlogs/")
                }
            ))
            .query(&params)
            .header("Authorization", auth.token.clone())
            .header("X-Flavortown-Ext-333", "true")
            .send()
            .await?;
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
            info!("Retrieved devlogs successfully.");
            let devlogs: DevlogVec = res.json().await?;
            if self.project_id.is_some() {
                println!(
                    "{}{}{}",
                    "Devlogs for project with ID: '".bold().cyan(), // TODO: Get project name instead of ID
                    self.project_id.as_ref().unwrap().bold().yellow(),
                    "'".bold().cyan()
                );
            }
            print_devlog_table(&devlogs.devlogs, &devlogs.pagination);
        }

        Ok(())
    }
}
