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

use crate::models::user_vec::UserList;
use crate::models::user_vec::Pagination;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use owo_colors::OwoColorize;

pub fn print_user_table(users: &[UserList], pagination: &Pagination) {
    if users.is_empty() {
        println!("No users found.");
        return;
    }
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "ID",
            "Display Name",
            "Cookies"
        ]);
    for user in users {
        let id = user.id.to_string();
        let display_name = user.display_name.clone();
        let cookies = user.cookies.to_string();
        table.add_row(vec![
            id,
            display_name,
            cookies
        ]);
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
