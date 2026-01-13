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

use crate::models::user_vec::Pagination;
use crate::models::user_vec::UserList;
use crate::commands::user::list::UserFields;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use owo_colors::OwoColorize;

fn format_u32(u: u32) -> Cell { // for id, cookies
    Cell::new(u.to_string())
}
fn format_str(s: &str) -> Cell { // for slack id, display name, avatar
    Cell::new(s.to_string())
}
fn format_project_ids(project_ids: &[u32]) -> Cell {
    let formatted = if project_ids.is_empty() {
        "None".to_string()
    } else if project_ids.len() > 5 {
        let mut ids: Vec<String> = project_ids.iter().take(5).map(|id| id.to_string()).collect();
        ids.push("...".to_string());
        ids.join(", ")
    } else {
        project_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(", ")
    };
    Cell::new(formatted)
}

pub fn print_user_table(users: &[UserList], pagination: &Pagination, fields: Vec<UserFields>) {
    if users.is_empty() {
        println!("No users found.");
        return;
    }
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Disabled);

    // Build header
    let mut header = Vec::<&str>::new();
    for field in &fields {
        match field {
            UserFields::Id => { header.push("ID"); },
            UserFields::SlackId => { header.push("Slack ID"); },
            UserFields::DisplayName => { header.push("Display Name"); },
            UserFields::Avatar => { header.push("Avatar"); },
            UserFields::ProjectIds => { header.push("Project IDs"); },
            UserFields::Cookies => { header.push("Cookies"); },
        }
    }
    table.set_header(header);
    for user in users {
        let mut row: Vec<Cell> = Vec::new();
        for field in &fields {
            match field {
                UserFields::Id => row.push(format_u32(user.id)),
                UserFields::SlackId => row.push(format_str(&user.slack_id)),
                UserFields::DisplayName => row.push(format_str(&user.display_name)),
                UserFields::Avatar => row.push(format_str(&user.avatar)),
                UserFields::ProjectIds => row.push(format_project_ids(&user.project_ids)),
                UserFields::Cookies => row.push(format_u32(user.cookies)),
            }
        }
        table.add_row(row);
    }

    let footer_text = if let Some(next) = pagination.next_page {
        format!(
            "Page {}/{} | Total users: {} | Next page: {}",
            pagination.current_page, pagination.total_pages, pagination.total_count, next
        )
    } else {
        format!(
            "Page {}/{} | Total users: {}",
            pagination.current_page, pagination.total_pages, pagination.total_count
        )
    };

    println!("{}", table);
    println!("\r{}", footer_text.bold().cyan());
    println!();
}
