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
use crate::helpers::print_user::print_user;
use crate::models::authdata::AuthData;
use crate::models::user::User;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};

#[derive(Debug, Args)]
pub struct UserGet {
    // Defines get user command (level 3)
    /// The user ID to retrieve. Defaults to you if not supplied
    pub user_id: Option<u32>,

    /// Returns data as raw JSON
    #[clap(long, conflicts_with = "resolve")]
    pub json: bool,

    /// Resolves project IDs, or when specified twice, devlog ids as well (may be VERY long)
    #[clap(long, short, alias="long", alias="detailed", conflicts_with = "json", action = clap::ArgAction::Count)]
    pub resolve: u8,
}

impl UserGet {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!("Executing user get command for user ID: {:?}", self.user_id);
        let auth: AuthData = get_key()?;
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Retrieving user...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let client = reqwest::Client::new();
        let url = format!(
            "https://flavortown.hackclub.com/api/v1/users/{}",
            if let Some(user_id) = &self.user_id {
                user_id.to_string()
            } else {
                "me".to_string()
            }
        );
        debug!("Sending GET request to {}", url);
        let res = client
            .get(&url)
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
                    404 => "Is the user ID correct?",
                    _ => "Please try again later.",
                }
            );
        } else {
            spinner.finish_and_clear();
            info!("Retrieved user successfully.");
            if self.json {
                let user_json = res.text().await?;
                debug!("Returning raw JSON data");
                println!("{}", user_json);
            } else {
                let user: User = res.json().await?;
                debug!("Successfully parsed user data");
                print_user(&user, self.resolve).await;
            }
        }

        Ok(())
    }
}
