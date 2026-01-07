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

use crate::models::store::AghContents;
use crate::models::store::Store;
use owo_colors::OwoColorize;
use textwrap::fill;

pub fn print_store(i: &Store, short: bool, detailed: bool) {
    println!("{}\n{}", i.name.bold().yellow(), "-".repeat(40));
    println!("{:<20}: {}", "ID".blue(), i.id);
    let stock_str = i
        .stock
        .map(|s| s.to_string())
        .unwrap_or_else(|| "∞".to_string());
    println!("{:<20}: {}", "Stock".blue(), stock_str);
    println!("{:<20}: {}", "Type".blue(), i.type_);
    println!("{:<20}: {}", "Description".blue(), i.description);

    if let Some(long_desc) = &i.long_description
        && !long_desc.is_empty()
    {
        println!("\n{}", "Long Description:".bold().cyan());
        println!("{}", fill(long_desc, 72));
    }

    println!("\n{}", "Regional Info:".bold().cyan());
    println!(
        "{:<20}: {} {}{}",
        "🇦🇺 Australia".blue(),
        "Enabled?".dimmed(),
        i.enabled.enabled_au,
        if i.enabled.enabled_au {
            format!(" {} {}", "Price:".dimmed(), i.ticket_cost.au)
        } else {
            "".to_string()
        }
    );
    println!(
        "{:<20}: {} {}{}",
        "🇨🇦 Canada".blue(),
        "Enabled?".dimmed(),
        i.enabled.enabled_ca,
        if i.enabled.enabled_ca {
            format!(" {} {}", "Price:".dimmed(), i.ticket_cost.ca)
        } else {
            "".to_string()
        }
    );
    println!(
        "{:<20}: {} {}{}",
        "🇪🇺 Europe".blue(),
        "Enabled?".dimmed(),
        i.enabled.enabled_eu,
        if i.enabled.enabled_eu {
            format!(" {} {}", "Price:".dimmed(), i.ticket_cost.eu)
        } else {
            "".to_string()
        }
    );
    println!(
        "{:<20}: {} {}{}",
        "🇮🇳 India".blue(),
        "Enabled?".dimmed(),
        i.enabled.enabled_in,
        if i.enabled.enabled_in {
            format!(" {} {}", "Price:".dimmed(), i.ticket_cost.in_)
        } else {
            "".to_string()
        }
    );
    println!(
        "{:<20}: {} {}{}",
        "🇬🇧 United Kingdom".blue(),
        "Enabled?".dimmed(),
        i.enabled.enabled_uk,
        if i.enabled.enabled_uk {
            format!(" {} {}", "Price:".dimmed(), i.ticket_cost.uk)
        } else {
            "".to_string()
        }
    );
    println!(
        "{:<20}: {} {}{}",
        "🇺🇸 United States".blue(),
        "Enabled?".dimmed(),
        i.enabled.enabled_us,
        if i.enabled.enabled_us {
            format!(" {} {}", "Price:".dimmed(), i.ticket_cost.us)
        } else {
            "".to_string()
        }
    );
    println!(
        "{:<20}: {} {}{}",
        "🇺🇳 Global".blue(),
        "Enabled?".dimmed(),
        i.enabled.enabled_xx,
        if i.enabled.enabled_xx {
            format!(" {} {}", "Price:".dimmed(), i.ticket_cost.xx)
        } else {
            "".to_string()
        }
    );

    if !short {
        println!("\n{}", "Buying Info:".bold().cyan());
        println!("{:<20}: {}", "Limited?".blue(), i.limited);
        println!("{:<20}: {}", "Stock".blue(), stock_str);
        if i.sale_percentage.is_some() {
            println!(
                "{:<20}: {}",
                "Sale Percentage".blue(),
                i.sale_percentage.unwrap()
            );
        }
        if i.max_qty.is_some() {
            println!("{:<20}: {}", "Max Qty".blue(), i.max_qty.unwrap());
        }
        println!(
            "{:<20}: {}",
            "One Per Person Ever?".blue(),
            i.one_per_person_ever
        );
        println!("{:<20}: {}", "Buyable By Self?".blue(), i.buyable_by_self);
        if let Some(tag) = &i.accessory_tag
            && !tag.is_empty()
        {
            println!("{:<20}: {}", "Accessory Tag".blue(), tag);
        }

        if !i.attached_shop_item_ids.is_empty()
            && i.attached_shop_item_ids.iter().any(|id| id.is_some())
        {
            println!("\n{}", "Attached Item IDs:".bold().cyan());
            for id in i.attached_shop_item_ids.iter().flatten() {
                println!("  - {}", id);
            }
        }

        println!("\n{}", "Image URL:".bold().cyan());
        println!("{}", i.image_url);
    }

    if detailed {
        println!("\n{}", "Random metadata:".bold().cyan());
        if !i.old_prices.is_empty() {
            println!(
                "{:<20}: {}",
                "Old Prices".blue(),
                i.old_prices
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
        println!("{:<20}: {}", "Show In Carousel?".blue(), i.show_in_carousel);
        if !matches!(&i.agh_contents, AghContents::Null)
            && !matches!(&i.agh_contents, AghContents::String(s) if s.is_empty())
        {
            println!("\n{}", "AGH Contents:".bold().cyan());
            match &i.agh_contents {
                AghContents::Choice(c) => {
                    println!("{:<20}: Base quantity: {}", "Choice".blue(), c.base_qty);
                    for choice in &c.choice {
                        println!("{:<20}  - {}", "", choice);
                    }
                }
                AghContents::Items(items) => {
                    println!("{:<20}: Items:", "Items".blue());
                    for item in items {
                        println!(
                            "{:<20}  - SKU: {}, Quantity: {}",
                            "", item.sku, item.quantity
                        );
                    }
                }
                AghContents::String(s) => println!("{:<20}: {}", "String".blue(), s),
                AghContents::Null => {}
            }
        }
    }
}
