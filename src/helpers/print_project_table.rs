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

use crate::commands::project::list::ProjectFields;
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
    text.chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect()
}

fn format_id(u: u32) -> Cell {
    Cell::new(u.to_string())
}
fn format_title(s: &str) -> Cell {
    let title = if s.chars().count() > 30 {
        format!("{}...", s.chars().take(27).collect::<String>())
    } else {
        s.to_string()
    };
    Cell::new(sanitize(&title))
}
fn format_description(s: &str) -> Cell {
    let description = if s.chars().count() > 50 {
        format!("{}...", s.chars().take(47).collect::<String>())
    } else {
        s.to_string()
    };
    Cell::new(sanitize(&description))
}
fn format_status(s: &str) -> Cell {
    Cell::new(s)
}
fn format_url(s: &str) -> Cell {
    let display = if s.is_empty() {
        "-".to_string()
    } else {
        s.to_string()
    };
    Cell::new(sanitize(&display))
}
fn format_time_cell(dt: &str) -> Cell {
    Cell::new(format_time(dt))
}
fn format_devlog_ids(ids: &[u32]) -> Cell {
    let display = if ids.is_empty() {
        "-".to_string()
    } else {
        ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    };
    Cell::new(sanitize(&display))
}

pub fn print_project_table(
    projects: &[Project],
    pagination: &Pagination,
    fields: Vec<ProjectFields>,
) {
    if projects.is_empty() {
        println!("No projects found.");
        return;
    }
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let mut header = Vec::<&str>::new();
    for field in &fields {
        match field {
            ProjectFields::Id => header.push("ID"),
            ProjectFields::Title => header.push("Title"),
            ProjectFields::Description => header.push("Description"),
            ProjectFields::ShipStatus => header.push("Status"),
            ProjectFields::RepoUrl => header.push("Repo"),
            ProjectFields::DemoUrl => header.push("Demo"),
            ProjectFields::ReadmeUrl => header.push("Readme"),
            ProjectFields::CreatedAt => header.push("Created"),
            ProjectFields::UpdatedAt => header.push("Updated"),
            ProjectFields::DevlogIds => header.push("Devlog IDs"),
        }
    }
    table.set_header(header);
    for project in projects {
        let mut row: Vec<Cell> = Vec::new();
        for field in &fields {
            match field {
                ProjectFields::Id => row.push(format_id(project.id)),
                ProjectFields::Title => row.push(format_title(&project.title)),
                ProjectFields::Description => row.push(format_description(&project.description)),
                ProjectFields::ShipStatus => row.push(format_status(&project.ship_status)),
                ProjectFields::RepoUrl => row.push(format_url(&project.repo_url)),
                ProjectFields::DemoUrl => row.push(format_url(&project.demo_url)),
                ProjectFields::ReadmeUrl => row.push(format_url(&project.readme_url)),
                ProjectFields::CreatedAt => row.push(format_time_cell(&project.created_at)),
                ProjectFields::UpdatedAt => row.push(format_time_cell(&project.updated_at)),
                ProjectFields::DevlogIds => row.push(format_devlog_ids(&project.devlog_ids)),
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
