// Copyright (C) 2025 Lordseriouspig
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
use chrono::{DateTime, Local};
use owo_colors::OwoColorize;
use textwrap::fill;

fn format_time(dt: &str) -> String {
    let dt = DateTime::parse_from_rfc3339(dt).unwrap();
    let local_dt = dt.with_timezone(&Local);
    local_dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn print_project(p: &Project) {
    println!("{}\n{}", p.title.bold().yellow(), "-".repeat(40));
    println!("{:<12}: {}", "ID".blue(), p.id);
    println!("{:<12}: {}", "Status".blue(), p.ship_status);
    println!("{:<12}: {}", "Created".blue(), format_time(&p.created_at));
    println!("{:<12}: {}", "Updated".blue(), format_time(&p.updated_at));

    println!("\n{}", "Description:".bold().cyan());
    println!("{}", fill(&p.description, 72));

    println!("\n{}", "Links:".bold().cyan());
    println!(
        "{:<7} {}",
        "Repo".blue(),
        if p.repo_url.is_empty() {
            "-"
        } else {
            &p.repo_url
        }
    );
    println!(
        "{:<7} {}",
        "Demo".blue(),
        if p.demo_url.is_empty() {
            "-"
        } else {
            &p.demo_url
        }
    );
    println!(
        "{:<7} {}",
        "Readme".blue(),
        if p.readme_url.is_empty() {
            "-"
        } else {
            &p.readme_url
        }
    );

    println!("\n{}", "Devlog IDs:".bold().cyan());
    if p.devlog_ids.is_empty() {
        println!("- None -");
    } else {
        for id in &p.devlog_ids {
            println!("- {}", id);
        }
    }
}
