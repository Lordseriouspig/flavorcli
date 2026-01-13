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

use crate::{field, heading, list, long_text, title};
use crate::{
    helpers::{print_devlog::print_devlog, resolve_devlogs::resolve_devlogs},
    models::project::Project,
};
use chrono::{DateTime, Local};
use log::warn;
use owo_colors::OwoColorize;

fn format_time(dt: &str) -> String {
    let dt = DateTime::parse_from_rfc3339(dt).unwrap();
    let local_dt = dt.with_timezone(&Local);
    local_dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub async fn print_project(p: &Project, resolve: bool) {
    title!(p.title);
    field!("ID", p.id);
    field!("Status", p.ship_status);
    field!("Created", format_time(&p.created_at));
    field!("Updated", format_time(&p.updated_at));
    long_text!("Description", &p.description);

    heading!("Links:");
    field!(
        "Repo",
        if p.repo_url.is_empty() {
            "-"
        } else {
            &p.repo_url
        }
    );
    field!(
        "Demo",
        if p.demo_url.is_empty() {
            "-"
        } else {
            &p.demo_url
        }
    );
    field!(
        "Readme",
        if p.readme_url.is_empty() {
            "-"
        } else {
            &p.readme_url
        }
    );

    if resolve {
        heading!("Devlogs:");
        match resolve_devlogs(&p.devlog_ids).await {
            Ok(devlogs) => {
                if devlogs.is_empty() {
                    println!("- None -");
                } else {
                    for devlog in devlogs {
                        print_devlog(&devlog, true);
                        println!();
                    }
                }
            }
            Err(e) => {
                warn!("{} {}", "Unable to resolve devlogs:".red(), e);
                heading!("Devlog IDs:");
                if p.devlog_ids.is_empty() {
                    println!("- None -");
                } else {
                    list!(&p.devlog_ids);
                }
            }
        }
    } else {
        println!("\n{}", "Devlog IDs:".bold().cyan());
        if p.devlog_ids.is_empty() {
            println!("- None -");
        } else {
            list!(&p.devlog_ids);
        }
    }
}
