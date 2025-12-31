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

use crate::models::authdata::AuthData;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use keyring::Entry;
use log::{info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Args)]
pub struct AuthSet {
    // Defines set command (level 3)
    /// Your Flavortown ID (find it in the URL of your profile)
    pub user_id: u64,
    /// Your Flavortown authentication token
    pub token: String,
    /// Disables token verification (not recommended)
    #[clap(long)]
    pub no_verify: bool,
}

impl AuthSet {
    pub async fn execute(&self) -> anyhow::Result<()> {
        let entry = Entry::new("flavorcli", "auth_token")?;

        // Verify Token
        if !self.no_verify {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::with_template("{spinner} {msg}")?
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );
            spinner.set_message("Verifying token...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(80));

            let client = reqwest::Client::new();
            let res = client
                .get(&format!(
                    "https://flavortown.hackclub.com/api/v1/users/{}",
                    self.user_id
                ))
                .header("Authorization", self.token.clone())
                .header("X-Flavortown-Ext-333", "true")
                .send()
                .await?;
            if !res.status().is_success() {
                spinner.finish_and_clear();
                anyhow::bail!(
                    "Token verification failed with status: {}. Are you sure your token is correct?",
                    res.status()
                );
            } else {
                spinner.finish_and_clear();
                info!(
                    "Token verified. Welcome, {}!",
                    res.json::<serde_json::Value>()
                        .await?
                        .get("display_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("user")
                );
            }
        } else {
            warn!("Token verification is disabled.");
        }

        let auth = AuthData {
            token: self.token.clone(),
            user_id: self.user_id,
        };

        let json = serde_json::to_string(&auth)?;

        entry.set_password(&json)?;
        info!("Authentication token set successfully.");
        Ok(())
    }
}
