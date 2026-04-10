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

use crate::helpers::print_devlog::print_devlog;
use crate::models::devlog::Devlog;
use crate::models::authdata::AuthData;
use crate::models::session::Session;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};

#[derive(Debug, Args)]
pub struct DevlogGet {
    // Defines get devlog command (level 4)
    /// The devlog ID to retrieve
    pub devlog_id: u32,
    /// Returns data as raw JSON
    #[clap(long, conflicts_with = "short")]
    pub json: bool,
    /// Omits the devlog's metadata
    #[clap(long, short, conflicts_with = "json")]
    pub short: bool,
}

impl DevlogGet {
    pub async fn execute(&self, session: &Session, auth: &AuthData) -> anyhow::Result<()> {
        debug!(
            "Executing devlog get command (devlog_id: {})",
            self.devlog_id
        );
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Retrieving devlog...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let url = format!(
            "https://flavortown.hackclub.com/api/v1/devlogs/{}",
            self.devlog_id
        );
        let res = session
            .get(&url, auth.token.clone(), None)
            .await?;
        if !res.status().is_success() {
            spinner.finish_and_clear();
            anyhow::bail!(
                "Request failed with status: {}. {}",
                res.status(),
                match res.status().as_u16() {
                    401 => "Is your token correct?",
                    404 => "Is the project ID and/or devlog ID correct?",
                    _ => "Please try again later.",
                }
            );
        } else {
            spinner.finish_and_clear();
            info!("Retrieved devlog successfully.");
            if self.json {
                let devlog_json = res.text().await?;
                debug!("Returning raw JSON data");
                println!("{}", devlog_json);
            } else {
                let devlog: Devlog = res.json().await?;
                debug!("Successfully parsed devlog data");
                print_devlog(&devlog, self.short);
            }
        }

        Ok(())
    }
}
