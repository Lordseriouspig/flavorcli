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
use chrono::{DateTime, Local};
use owo_colors::OwoColorize;
use textwrap::fill;

fn format_time(dt: &str) -> String {
    let dt = DateTime::parse_from_rfc3339(dt).unwrap();
    let local_dt = dt.with_timezone(&Local);
    local_dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn format_duration(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

pub fn print_devlog(d: &Devlog) {
    println!(
        "{}\n{}",
        format!("Devlog #{}", d.id).bold().yellow(),
        "-".repeat(40)
    );
    println!(
        "{:<12}: {}",
        "Comments".blue(),
        d.comments_count
    );
    println!(
        "{:<12}: {}",
        "Time".blue(),
        format_duration(d.duration_seconds)
    );
    println!("{:<12}: {}", "Likes".blue(), d.likes_count);
    println!("{:<12}: {}", "Created".blue(), format_time(&d.created_at));
    println!("{:<12}: {}", "Updated".blue(), format_time(&d.updated_at));

    println!("\n{}", "Body:".bold().cyan());
    println!("{}", fill(&d.body, 72));

    if d.scrapbook_url.is_some() {
        println!("\n{}", "Scrapbook:".bold().cyan());
        println!("{:<12} {}", "URL".blue(), d.scrapbook_url.as_ref().unwrap());
    }
}
