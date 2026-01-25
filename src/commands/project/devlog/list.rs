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
use log::{debug, info};
#[derive(Debug, Args)]
pub struct ProjectDevlogList {
    /// Page number for pagination. Defaults to 1.
    #[clap(long, short)]
    pub page: Option<u32>,

    /// Returns data as raw JSON
    #[clap(long)]
    pub json: bool,

    /// Fields to output in the table (advanced)
    #[clap(
        long,
        value_enum,
        conflicts_with = "json",
        value_delimiter = ',',
        default_value = "id,body,duration,likes-count,comments-count,updated-at"
    )]
    pub fields: Vec<DevlogFields>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevlogFields {
    Id,
    Body,
    CommentsCount,
    Duration,
    LikesCount,
    ScrapbookUrl,
    CreatedAt,
    UpdatedAt,
}

impl ProjectDevlogList {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!(
            "Executing devlog list command (page: {:?})",
            self.page
        );
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
        let url = format!(
            "https://flavortown.hackclub.com/api/v1/devlogs/"
        );
        debug!("Sending GET request to {} with params: {:?}", url, params);
        let res = client
            .get(&url)
            .query(&params)
            .header("Authorization", auth.token)
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
            info!("Retrieved devlogs successfully.");
            if self.json {
                let devlogs_json = res.text().await?;
                debug!("Returning raw JSON data");
                println!("{}", devlogs_json);
            } else {
                let devlogs: DevlogVec = res.json().await?;
                debug!("Successfully parsed {} devlogs", devlogs.devlogs.len());
                print_devlog_table(&devlogs.devlogs, &devlogs.pagination, &self.fields);
            }
        }

        Ok(())
    }
}
