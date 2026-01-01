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

use crate::models::project::Project;
use crate::models::project_vec::Pagination;
use chrono::{DateTime, Local};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use owo_colors::OwoColorize;

fn format_time(dt: &str) -> String {
    let dt = DateTime::parse_from_rfc3339(dt).unwrap();
    let local_dt = dt.with_timezone(&Local);
    local_dt.format("%Y-%m-%d %H:%M").to_string()
}

fn sanitize(text: &str) -> String {
    text.replace('\n', " ").replace('\r', " ")
}

pub fn print_project_table(projects: &[Project], pagination: &Pagination) {
    if projects.is_empty() {
        println!("No projects found.");
        return;
    }
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["ID", "Title", "Description", "Updated"]);
    for project in projects {
        let id = project.id.to_string();
        let title = if project.title.len() > 30 {
            format!("{}...", &project.title[..27])
        } else {
            project.title.clone()
        };
        let desc = if project.description.len() > 50 {
            format!("{}...", &project.description[..47])
        } else {
            project.description.clone()
        };
        let updated = format_time(&project.updated_at);
        table.add_row(vec![
            id,
            sanitize(&title),
            sanitize(&desc),
            updated,
        ]);
    }
        let footer_text = if let Some(next) = pagination.next_page {
        format!(
            "Page {}/{} | Total results: {} | Next page: {}",
            pagination.current_page, pagination.total_pages, pagination.total_count, next
        )
    } else {
        format!(
            "Page {}/{} | Total items: {}",
            pagination.current_page, pagination.total_pages, pagination.total_count
        )
    };

    println!("{}", table);
    println!("\r{}", footer_text.bold().cyan());
    println!();
}
