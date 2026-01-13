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

use crate::models::devlog::Devlog;
use crate::models::devlog_vec::Pagination;
use crate::commands::project::devlog::list::DevlogFields;
use chrono::{DateTime, Local};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use owo_colors::OwoColorize;

fn format_time(dt: &str) -> String { // TODO: Move functions like this into a utils file or something
    let dt = DateTime::parse_from_rfc3339(dt).unwrap();
    let local_dt = dt.with_timezone(&Local);
    local_dt.format("%Y-%m-%d %H:%M").to_string()
}

fn format_duration(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

fn sanitize(text: &str) -> String {
    text.chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect()
}

fn format_id(u: u32) -> Cell {
    Cell::new(u.to_string())
}
fn format_body(s: &str) -> Cell {
    let body = if s.chars().count() > 50 {
        format!("{}...", s.chars().take(47).collect::<String>())
    } else {
        s.to_string()
    };
    Cell::new(sanitize(&body))
}
fn format_u32(u: u32) -> Cell {
    Cell::new(u.to_string())
}
fn format_duration_cell(seconds: u32) -> Cell {
    Cell::new(format_duration(seconds))
}
fn format_scrapbook_url(url: &Option<String>) -> Cell {
    let display_url = match url {
        Some(u) => u.clone(),
        None => "N/A".to_string(),
    };
    Cell::new(display_url)
}
fn format_time_cell(dt: &str) -> Cell {
    Cell::new(format_time(dt))
}

pub fn print_devlog_table(devlogs: &[Devlog], pagination: &Pagination, fields: Vec<DevlogFields>) {
    if devlogs.is_empty() {
        println!("No devlogs found.");
        return;
    }
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Build header
    let mut header = Vec::<&str>::new();
    for field in &fields {
        match field {
            DevlogFields::Id => { header.push("ID"); },
            DevlogFields::Body => { header.push("Body"); },
            DevlogFields::CommentsCount => { header.push("Comments"); },
            DevlogFields::Duration => { header.push("Duration"); },
            DevlogFields::LikesCount => { header.push("Likes"); },
            DevlogFields::ScrapbookUrl => { header.push("Scrapbook URL"); },
            DevlogFields::CreatedAt => { header.push("Created"); },
            DevlogFields::UpdatedAt => { header.push("Updated"); },
        }
    }
    table.set_header(header);
    for devlog in devlogs {
        let mut row: Vec<Cell> = Vec::new();
        for field in &fields {
            match field {
                DevlogFields::Id => row.push(format_id(devlog.id)),
                DevlogFields::Body => row.push(format_body(&devlog.body)),
                DevlogFields::CommentsCount => row.push(format_u32(devlog.comments_count)),
                DevlogFields::Duration => row.push(format_duration_cell(devlog.duration_seconds)),
                DevlogFields::LikesCount => row.push(format_u32(devlog.likes_count)),
                DevlogFields::ScrapbookUrl => row.push(format_scrapbook_url(&devlog.scrapbook_url)),
                DevlogFields::CreatedAt => row.push(format_time_cell(&devlog.created_at)),
                DevlogFields::UpdatedAt => row.push(format_time_cell(&devlog.updated_at)),
            }
        }
        table.add_row(row);
    }
    let footer_text = if let Some(next) = pagination.next_page {
        format!(
            "Page {}/{} | Total results: {} | Next page: {}",
            pagination.current_page, pagination.total_pages, pagination.total_count, next
        )
    } else {
        format!(
            "Page {}/{} | Total results: {}",
            pagination.current_page, pagination.total_pages, pagination.total_count
        )
    };

    println!("{}", table);
    println!("\r{}", footer_text.bold().cyan());
    println!();
}
