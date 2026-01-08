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

use crate::models::store::Store;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

fn color_cell(enabled: bool, value: f64) -> Cell {
    if enabled {
        Cell::new(value.to_string()).fg(Color::Green)
    } else {
        Cell::new("✘").fg(Color::Red)
    }
}

fn sanitize(text: &str) -> String {
    text.chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect()
}

pub fn print_store_table(mut items: Vec<Store>, region: Option<impl AsRef<str>>) {
    items.sort_by_key(|item| item.id);
    if items.is_empty() {
        println!("No items found.");
        return;
    }

    let region_filter = region.as_ref().map(|r| r.as_ref().to_uppercase());
    let show_all_regions = region_filter.is_none();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Disabled);

    // Build header dynamically based on region filter
    let mut header = vec!["ID", "Name", "Description", "Stock"];

    if show_all_regions {
        header.extend_from_slice(&["AU", "CA", "EU", "IN", "UK", "US", "XX"]);
    } else if let Some(ref region_code) = region_filter {
        header.push(region_code.as_str());
    }

    header.extend_from_slice(&["Type", "Attached to"]);
    table.set_header(header);

    for item in items {
        let id = item.id.to_string();
        let name = if item.name.len() > 30 {
            format!("{}...", &item.name[..27])
        } else {
            item.name.clone()
        };
        let desc = if item.description.len() > 50 {
            format!("{}...", &item.description[..47])
        } else {
            item.description.clone()
        };
        let stock = item
            .stock
            .map(|s| s.to_string())
            .unwrap_or_else(|| "∞".to_string());
        let type_ = item
            .type_
            .as_str()
            .strip_prefix("ShopItem::")
            .unwrap_or(item.type_.as_str())
            .to_string();
        let attatched_to = item
            .attached_shop_item_ids
            .iter()
            .filter_map(|id| id.map(|i| i.to_string()))
            .collect::<Vec<String>>()
            .join(", ");

        let mut row = vec![
            Cell::new(id),
            Cell::new(sanitize(&name)),
            Cell::new(sanitize(&desc)),
            Cell::new(stock),
        ];

        // Add region columns based on filter
        if show_all_regions {
            row.extend_from_slice(&[
                color_cell(item.enabled.enabled_au, item.ticket_cost.au),
                color_cell(item.enabled.enabled_ca, item.ticket_cost.ca),
                color_cell(item.enabled.enabled_eu, item.ticket_cost.eu),
                color_cell(item.enabled.enabled_in, item.ticket_cost.in_),
                color_cell(item.enabled.enabled_uk, item.ticket_cost.uk),
                color_cell(item.enabled.enabled_us, item.ticket_cost.us),
                color_cell(item.enabled.enabled_xx, item.ticket_cost.xx),
            ]);
        } else if let Some(ref region_code) = region_filter {
            match region_code.as_str() {
                "AU" => row.push(color_cell(item.enabled.enabled_au, item.ticket_cost.au)),
                "CA" => row.push(color_cell(item.enabled.enabled_ca, item.ticket_cost.ca)),
                "EU" => row.push(color_cell(item.enabled.enabled_eu, item.ticket_cost.eu)),
                "IN" => row.push(color_cell(item.enabled.enabled_in, item.ticket_cost.in_)),
                "UK" => row.push(color_cell(item.enabled.enabled_uk, item.ticket_cost.uk)),
                "US" => row.push(color_cell(item.enabled.enabled_us, item.ticket_cost.us)),
                "XX" => row.push(color_cell(item.enabled.enabled_xx, item.ticket_cost.xx)),
                _ => {} // Unknown region, skip column
            }
        }
        row.extend_from_slice(&[Cell::new(type_), Cell::new(attatched_to)]);
        table.add_row(row);
    }

    println!("{}", table);
    println!();
}
