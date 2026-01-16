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
use crate::helpers::print_store_table::print_store_table;
use crate::models::authdata::AuthData;
use crate::models::store::Store;
use anyhow;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};

#[derive(Debug, Args)]
pub struct StoreList {
    // Defines list store items command (level 3)
    /// Returns data as raw JSON
    #[clap(long, conflicts_with_all = ["region", "fields", "sort"])]
    pub json: bool,

    /// Region column to show
    #[clap(long, value_enum, conflicts_with = "json")]
    pub region: Option<Regions>,

    /// Fields to output in the table (advanced)
    #[clap(
        long,
        value_enum,
        conflicts_with = "json",
        value_delimiter = ',',
        default_value = "id,name,description,stock,regional,type,attached-to"
    )]
    pub fields: Vec<StoreFields>,

    /// Sort the table output
    #[clap(
        long,
        value_enum,
        conflicts_with = "json",
        default_value = "id",
        requires_if("regional", "sort_region")
    )]
    pub sort: SortFields,

    /// Choose the region to sort by if you selected "regional" as the sort order
    #[clap(long, value_enum, requires = "sort", conflicts_with = "json")]
    pub sort_region: Option<Regions>,

    /// Choose the direction of sort order
    #[clap(long, value_enum, requires = "sort", default_value = "asc")]
    pub sort_order: SortOrder,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreFields {
    Id,
    Name,
    Description,
    Stock,
    Type,
    AttachedTo,
    Limited,
    BuyableBySelf,
    ShowInCarousel,
    AccessoryTag,
    LongDescription,
    ImageUrl,
    MaxQty,
    OnePerPersonEver,
    SalePercentage,
    Regional,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortFields {
    Id,
    Name,
    Stock,
    Type,
    Limited,
    BuyableBySelf,
    ShowInCarousel,
    MaxQty,
    OnePerPersonEver,
    SalePercentage,
    Regional,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum Regions {
    #[value(name = "australia")]
    Au,
    #[value(name = "canada")]
    Ca,
    #[value(name = "europe")]
    Eu,
    #[value(name = "india")]
    In,
    #[value(name = "united-kingdom")]
    Uk,
    #[value(name = "united-states")]
    Us,
    #[value(name = "world")]
    Xx,
}

impl StoreList {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!("Executing store list command");
        let auth: AuthData = get_key()?;
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Retrieving items...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let client = reqwest::Client::new();
        // this is where to put params in the future (let params =)
        debug!("Sending GET request to https://flavortown.hackclub.com/api/v1/store");
        let res = client
            .get("https://flavortown.hackclub.com/api/v1/store")
            //.query(&params)
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
            info!("Retrieved items successfully.");
            if self.json {
                let items_json = res.text().await?;
                debug!("Returning raw JSON data");
                println!("{}", items_json);
            } else {
                let items: Vec<Store> = res.json().await?;
                debug!("Successfully parsed {} store items", items.len());
                print_store_table(
                    items,
                    self.region.map(|r| format!("{:?}", r).to_lowercase()),
                    self.fields.clone(),
                    self.sort,
                    self.sort_order,
                    self.sort_region.map(|r| format!("{:?}", r).to_uppercase()),
                );
            }
        }

        Ok(())
    }
}
