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

pub fn print_devlog_table(devlogs: &[Devlog], pagination: &Pagination) {
    if devlogs.is_empty() {
        println!("No devlogs found.");
        return;
    }
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["ID", "Body", "Time", "Likes", "Comments", "Updated"]);
    for devlog in devlogs {
        let id = devlog.id.to_string();
        let body = if devlog.body.chars().count() > 50 {
            format!("{}...", devlog.body.chars().take(47).collect::<String>())
        } else {
            devlog.body.clone()
        };
        let time = format_duration(devlog.duration_seconds);
        let likes = devlog.likes_count.to_string();
        let comments = devlog.comments_count.to_string();
        let updated = format_time(&devlog.updated_at);
        table.add_row(vec![id, sanitize(&body), time, likes, comments, updated]);
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
