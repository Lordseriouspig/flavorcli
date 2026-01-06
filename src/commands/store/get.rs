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
use crate::helpers::print_store::print_store;
use crate::models::authdata::AuthData;
use crate::models::store::Store;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, debug};

#[derive(Debug, Args)]
pub struct StoreGet {
    // Defines get store item command (level 3)
    /// The store item ID to retrieve
    pub item_id: u64, // TODO: --short, --detailed, --json
}

impl StoreGet {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!("Executing store get command for item ID: {}", self.item_id);
        let auth: AuthData = get_key()?;
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Retrieving store item...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let client = reqwest::Client::new();
        let url = format!(
            "https://flavortown.hackclub.com/api/v1/store/{}",
            self.item_id
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
                    404 => "Is the item ID correct?",
                    _ => "Please try again later.",
                }
            );
        } else {
            spinner.finish_and_clear();
            info!("Retrieved store item successfully.");
            let store_item: Store = res.json().await?;
            debug!("Successfully parsed store item data");
            print_store(&store_item);
        }

        Ok(())
    }
}
