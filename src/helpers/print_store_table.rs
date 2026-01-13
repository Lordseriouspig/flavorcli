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

use crate::commands::store::list::StoreFields;
use crate::commands::store::list::{SortFields, SortOrder};
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

fn format_id(id: u32) -> Cell {
    Cell::new(id.to_string())
}
fn format_name(name: &str) -> Cell {
    let formatted = if name.chars().count() > 30 {
        format!("{}...", name.chars().take(27).collect::<String>())
    } else {
        name.to_string()
    };
    Cell::new(sanitize(&formatted))
}
fn format_description(description: &str) -> Cell {
    let formatted = if description.chars().count() > 50 {
        format!("{}...", description.chars().take(47).collect::<String>())
    } else {
        description.to_string()
    };
    Cell::new(sanitize(&formatted))
}
fn format_stock(stock: Option<u32>) -> Cell {
    Cell::new(stock.map(|s| s.to_string()).unwrap_or_else(|| "∞".to_string()))
}
fn format_type(type_: &str) -> Cell {
    let formatted = type_.strip_prefix("ShopItem::").unwrap_or(type_).to_string();
    Cell::new(formatted)
}
fn format_attached_to(attached_shop_item_ids: &Vec<Option<u32>>) -> Cell {
    let joined = attached_shop_item_ids
        .iter()
        .filter_map(|id| id.map(|i| i.to_string()))
        .collect::<Vec<String>>()
        .join(", ");
    Cell::new(joined)
}
fn format_bool(value: bool) -> Cell {
    Cell::new(if value { "Yes" } else { "No" })
}
fn format_long_description(long_description: &Option<String>) -> Cell {
    let formatted = match long_description {
        Some(desc) => {
            if desc.chars().count() > 50 {
                format!("{}...", desc.chars().take(47).collect::<String>())
            } else {
                desc.to_string()
            }
        }
        None => "N/A".to_string(),
    };
    Cell::new(sanitize(&formatted))
}
fn format_max_qty(max_qty: &Option<u32>) -> Cell {
    Cell::new(match max_qty {
        Some(qty) => qty.to_string(),
        None => "∞".to_string(),
    })
}
fn format_sale_percentage(old_prices: &Vec<u32>) -> Cell {
    let formatted = if old_prices.len() >= 2 {
        let original_price = old_prices[old_prices.len() - 2] as f64;
        let current_price = old_prices[old_prices.len() - 1] as f64;
        if original_price > 0.0 {
            let discount = ((original_price - current_price) / original_price) * 100.0;
            format!("{:.2}%", discount)
        } else {
            "0.00%".to_string()
        }
    } else {
        "N/A".to_string()
    };
    Cell::new(formatted)
}
fn format_image_url(url: &str) -> Cell {
    let formatted = if url.chars().count() > 30 {
        format!("{}...", url.chars().take(27).collect::<String>())
    } else {
        url.to_string()
    };
    Cell::new(formatted)
}
fn format_accessory_tag(tag: &Option<String>) -> Cell {
    Cell::new(tag.clone().unwrap_or_else(|| "N/A".to_string()))
}
fn format_regional(
    item: &Store,
    region_filter: &Option<String>,
    show_all_regions: bool,
) -> Vec<Cell> {
    if show_all_regions {
        return vec![
            color_cell(item.enabled.enabled_au, item.ticket_cost.au),
            color_cell(item.enabled.enabled_ca, item.ticket_cost.ca),
            color_cell(item.enabled.enabled_eu, item.ticket_cost.eu),
            color_cell(item.enabled.enabled_in, item.ticket_cost.in_),
            color_cell(item.enabled.enabled_uk, item.ticket_cost.uk),
            color_cell(item.enabled.enabled_us, item.ticket_cost.us),
            color_cell(item.enabled.enabled_xx, item.ticket_cost.xx),
        ];
    }

    if let Some(region_code) = region_filter {
        match region_code.as_str() {
            "AU" => vec![color_cell(item.enabled.enabled_au, item.ticket_cost.au)],
            "CA" => vec![color_cell(item.enabled.enabled_ca, item.ticket_cost.ca)],
            "EU" => vec![color_cell(item.enabled.enabled_eu, item.ticket_cost.eu)],
            "IN" => vec![color_cell(item.enabled.enabled_in, item.ticket_cost.in_)],
            "UK" => vec![color_cell(item.enabled.enabled_uk, item.ticket_cost.uk)],
            "US" => vec![color_cell(item.enabled.enabled_us, item.ticket_cost.us)],
            "XX" => vec![color_cell(item.enabled.enabled_xx, item.ticket_cost.xx)],
            _ => Vec::new(),
        }
    } else {
        Vec::new()
    }
}

pub fn print_store_table(mut items: Vec<Store>, region: Option<impl AsRef<str>>, fields: Option<Vec<crate::commands::store::list::StoreFields>>, sort: crate::commands::store::list::SortFields, sort_order: crate::commands::store::list::SortOrder, sort_region: Option<impl AsRef<str>>) {
    match sort {
        SortFields::Id => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.id.clone());
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.id.clone()));
            }
        }
        SortFields::Name => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.name.clone());
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.name.clone()));
            }
        }
        SortFields::Stock => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.stock.unwrap_or(u32::MAX));
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.stock.unwrap_or(u32::MAX)));
            }
        }
        SortFields::Type => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.type_.clone());
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.type_.clone()));
            }
        }
        SortFields::Limited => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.limited);
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.limited));
            }
        }
        SortFields::BuyableBySelf => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.buyable_by_self);
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.buyable_by_self));
            }
        }
        SortFields::ShowInCarousel => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.show_in_carousel);
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.show_in_carousel));
            }
        }
        SortFields::MaxQty => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.max_qty.unwrap_or(u32::MAX));
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.max_qty.unwrap_or(u32::MAX)));
            }
        }
        SortFields::OnePerPersonEver => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.one_per_person_ever);
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.one_per_person_ever));
            }
        }
        SortFields::SalePercentage => {
            if sort_order == SortOrder::Asc {
                items.sort_by_key(|item| item.sale_percentage.unwrap_or(0));
            } else {
                items.sort_by_key(|item| std::cmp::Reverse(item.sale_percentage.unwrap_or(0)));
            }
        }
        SortFields::Regional => {
            if let Some(region_code) = sort_region {
                match region_code.as_ref() {
                    "AU" => {
                        if sort_order == SortOrder::Asc {
                            items.sort_by(|a, b| a.ticket_cost.au.partial_cmp(&b.ticket_cost.au).unwrap_or(std::cmp::Ordering::Equal));
                        } else {
                            items.sort_by(|a, b| b.ticket_cost.au.partial_cmp(&a.ticket_cost.au).unwrap_or(std::cmp::Ordering::Equal));
                        }
                    }
                    "CA" => {
                        if sort_order == SortOrder::Asc {
                            items.sort_by(|a, b| a.ticket_cost.ca.partial_cmp(&b.ticket_cost.ca).unwrap_or(std::cmp::Ordering::Equal));
                        } else {
                            items.sort_by(|a, b| b.ticket_cost.ca.partial_cmp(&a.ticket_cost.ca).unwrap_or(std::cmp::Ordering::Equal));
                        }
                    }
                    "EU" => {
                        if sort_order == SortOrder::Asc {
                            items.sort_by(|a, b| a.ticket_cost.eu.partial_cmp(&b.ticket_cost.eu).unwrap_or(std::cmp::Ordering::Equal));
                        } else {
                            items.sort_by(|a, b| b.ticket_cost.eu.partial_cmp(&a.ticket_cost.eu).unwrap_or(std::cmp::Ordering::Equal));
                        }
                    }
                    "IN" => {
                        if sort_order == SortOrder::Asc {
                            items.sort_by(|a, b| a.ticket_cost.in_.partial_cmp(&b.ticket_cost.in_).unwrap_or(std::cmp::Ordering::Equal));
                        } else {
                            items.sort_by(|a, b| b.ticket_cost.in_.partial_cmp(&a.ticket_cost.in_).unwrap_or(std::cmp::Ordering::Equal));
                        }
                    }
                    "UK" => {
                        if sort_order == SortOrder::Asc {
                            items.sort_by(|a, b| a.ticket_cost.uk.partial_cmp(&b.ticket_cost.uk).unwrap_or(std::cmp::Ordering::Equal));
                        } else {
                            items.sort_by(|a, b| b.ticket_cost.uk.partial_cmp(&a.ticket_cost.uk).unwrap_or(std::cmp::Ordering::Equal));
                        }
                    }
                    "US" => {
                        if sort_order == SortOrder::Asc {
                            items.sort_by(|a, b| a.ticket_cost.us.partial_cmp(&b.ticket_cost.us).unwrap_or(std::cmp::Ordering::Equal));
                        } else {
                            items.sort_by(|a, b| b.ticket_cost.us.partial_cmp(&a.ticket_cost.us).unwrap_or(std::cmp::Ordering::Equal));
                        }
                    }
                    "XX" => {
                        if sort_order == SortOrder::Asc {
                            items.sort_by(|a, b| a.ticket_cost.xx.partial_cmp(&b.ticket_cost.xx).unwrap_or(std::cmp::Ordering::Equal));
                        } else {
                            items.sort_by(|a, b| b.ticket_cost.xx.partial_cmp(&a.ticket_cost.xx).unwrap_or(std::cmp::Ordering::Equal));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

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

    if let Some(selected_fields) = fields {
        // Build table based on supplied fields
        let mut header = Vec::<&str>::new();
        // Get the fields the user supplies and add the headers
        for field in &selected_fields {
            match field {
                StoreFields::Id => header.push("ID"),
                StoreFields::Name => header.push("Name"),
                StoreFields::Description => header.push("Description"),
                StoreFields::Stock => header.push("Stock"),
                StoreFields::Type => header.push("Type"),
                StoreFields::AttachedTo => header.push("Attached to"),
                StoreFields::Limited => header.push("Limited"),
                StoreFields::BuyableBySelf => header.push("Buyable By Self"),
                StoreFields::ShowInCarousel => header.push("Show In Carousel"),
                StoreFields::AccessoryTag => header.push("Accessory Tag"),
                StoreFields::LongDescription => header.push("Long Description"),
                StoreFields::ImageUrl => header.push("Image URL"),
                StoreFields::MaxQty => header.push("Max Qty"),
                StoreFields::OnePerPersonEver => header.push("One Per Person Ever"),
                StoreFields::SalePercentage => header.push("Sale Percentage"),
                StoreFields::Regional => {
                    if show_all_regions {
                        header.extend_from_slice(&["AU", "CA", "EU", "IN", "UK", "US", "XX"]);
                    } else if let Some(ref region_code) = region_filter {
                        header.push(region_code.as_str());
                    }
                }
            }
        }
        table.set_header(header);
        for item in items {
            let mut row: Vec<Cell> = Vec::new();
            for field in &selected_fields {
                match field {
                    StoreFields::Id => row.push(format_id(item.id)),
                    StoreFields::Name => row.push(format_name(&item.name)),
                    StoreFields::Description => row.push(format_description(&item.description)),
                    StoreFields::Stock => row.push(format_stock(item.stock)),
                    StoreFields::Type => row.push(format_type(&item.type_)),
                    StoreFields::AttachedTo => row.push(format_attached_to(&item.attached_shop_item_ids)),
                    StoreFields::Limited => row.push(format_bool(item.limited)),
                    StoreFields::BuyableBySelf => row.push(format_bool(item.buyable_by_self)),
                    StoreFields::ShowInCarousel => row.push(format_bool(item.show_in_carousel)),
                    StoreFields::AccessoryTag => row.push(format_accessory_tag(&item.accessory_tag)),
                    StoreFields::LongDescription => row.push(format_long_description(&item.long_description)),
                    StoreFields::ImageUrl => row.push(format_image_url(&item.image_url)),
                    StoreFields::MaxQty => row.push(format_max_qty(&item.max_qty)),
                    StoreFields::OnePerPersonEver => row.push(format_bool(item.one_per_person_ever)),
                    StoreFields::SalePercentage => row.push(format_sale_percentage(&item.old_prices)),
                    StoreFields::Regional => row.extend(format_regional(&item, &region_filter, show_all_regions)),
                }
            }
            table.add_row(row);
        }
    } else {
        // Build table based on defaults
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
            let mut row = vec![
                format_id(item.id),
                format_name(&item.name),
                format_description(&item.description),
                format_stock(item.stock),
            ];

            row.extend_from_slice(&format_regional(&item, &region_filter, show_all_regions));
            row.extend_from_slice(&[format_type(&item.type_), format_attached_to(&item.attached_shop_item_ids)]);
            table.add_row(row);
        }
    }

    println!("{}", table);
    println!();
}
