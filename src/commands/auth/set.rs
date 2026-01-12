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
use crate::models::user::User;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use keyring::Entry;
use log::{debug, info, warn};

#[derive(Debug, Args)]
pub struct AuthSet {
    // Defines set command (level 3)
    /// Your Flavortown authentication token
    pub token: String,

    /// Your Flavortown ID (find it in the URL of your profile)
    #[clap(long)]
    pub user_id: Option<u64>,

    /// Disables token verification (not recommended)
    #[clap(long, requires("user_id"))]
    pub no_verify: bool,
}

impl AuthSet {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!(
            "Executing auth set command (user_id: {}, no_verify: {})",
            self.user_id.unwrap_or(0),
            self.no_verify
        );
        let entry = Entry::new("flavorcli", "auth_token")?;
        let mut user: Option<User> = None;

        // Verify Token
        if !self.no_verify {
            debug!("Verifying token with API");
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::with_template("{spinner} {msg}")?
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );
            spinner.set_message("Verifying token...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(80));

            let client = reqwest::Client::new();
            let url = "https://flavortown.hackclub.com/api/v1/users/me".to_string();
            debug!("Sending GET request to {}", url);
            let res = client
                .get(&url)
                .header("Authorization", self.token.clone())
                .header("X-Flavortown-Ext-333", "true")
                .send()
                .await?;
            debug!("Received response with status: {}", res.status());
            if !res.status().is_success() {
                spinner.finish_and_clear();
                anyhow::bail!(
                    "Token verification failed with status: {}. Are you sure your token is correct?",
                    res.status()
                );
            } else {
                spinner.finish_and_clear();
                user = Some(res.json().await?);
                info!(
                    "Token verified, welcome {}!",
                    user.as_ref().unwrap().display_name
                );
            }
        } else {
            warn!("Token verification is disabled.");
        }

        if self.user_id.is_some() { info!("Overriding user ID to: {}", self.user_id.unwrap()); };

        let auth = AuthData {
            token: self.token.clone(),
            user_id: self
                .user_id
                .or_else(|| { let id = user.as_ref().map(|u| u.id as u64); info!("Automagically determined user ID: {:?}", id); id })
                .ok_or_else(|| anyhow::anyhow!("No user ID found"))?,
        };

        let json = serde_json::to_string(&auth)?;

        entry.set_password(&json)?;
        println!("Authentication token set successfully.");
        Ok(())
    }
}
