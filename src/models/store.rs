// Copyright (C) 2026 Lordseriouspig
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

use serde::Deserialize;

/**
 * This struct will be the first thing to break if even someone breathes on the API the wrong way
 */

#[derive(Debug, Deserialize)]
pub struct AghChoice {
    // i hate that this has to exist
    pub choice: Vec<String>,
    pub base_qty: u32,
}

#[derive(Debug, Deserialize)]
pub struct AghItem {
    // same with this one
    pub sku: String,
    pub quantity: u32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AghContents {
    // the name of this field reflects my emotion when dealing with it
    Choice(AghChoice),
    Items(Vec<AghItem>),
    String(String),
    Null,
}

#[derive(Debug, Deserialize)]
pub struct Enabled {
    pub enabled_au: bool,
    pub enabled_ca: bool,
    pub enabled_eu: bool,
    pub enabled_in: bool,
    pub enabled_uk: bool,
    pub enabled_us: bool,
    pub enabled_xx: bool, // i will personally go over there and fix the docs if they are inconsitent again
}

#[derive(Debug, Deserialize)]
pub struct TicketCost {
    pub au: f64,
    pub ca: f64,
    pub eu: f64,
    #[serde(rename = "in")]
    pub in_: f64,
    pub uk: f64,
    pub us: f64,
    pub xx: f64,
}

#[derive(Debug, Deserialize)]
pub struct Store {
    // This could break really badly if one of the fields is optional but I thought it wasn't haha.
    pub id: u32,
    pub name: String,
    pub description: String,
    pub old_prices: Vec<u32>,
    pub limited: bool,
    pub stock: Option<u32>,
    #[serde(rename = "type")]
    pub type_: String,
    pub show_in_carousel: bool,
    pub accessory_tag: Option<String>,
    pub agh_contents: AghContents, // THIS CAN BE THREE DIFFERENT TYPES ON THE BACKEND WHYYYYYYYYYYYYY.
    pub attached_shop_item_ids: Vec<Option<u32>>,
    pub buyable_by_self: bool,
    pub long_description: Option<String>,
    pub max_qty: Option<u32>,
    pub one_per_person_ever: bool,
    pub sale_percentage: Option<u64>,
    pub image_url: String,
    pub enabled: Enabled,
    pub ticket_cost: TicketCost,
}
