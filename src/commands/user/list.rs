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
use crate::helpers::print_user_table::print_user_table;
use crate::models::authdata::AuthData;
use crate::models::user_vec::UserVec;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;

#[derive(Debug, Args)]
pub struct UserList {
    // Defines list users command (level 3)
    /// Page number for pagination
    #[clap(long, short)]
    pub page: Option<u32>,
}

impl UserList {
    pub async fn execute(&self) -> anyhow::Result<()> {
        let auth: AuthData = get_key()?;
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Retrieving users...");
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
            .get("https://flavortown.hackclub.com/api/v1/users")
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
            info!("Retrieved users successfully.");
            let users: UserVec = res.json().await?;
            print_user_table(&users.users, &users.pagination);
        }

        Ok(())
    }
}
