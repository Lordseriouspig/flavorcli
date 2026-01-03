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

use crate::models::user::User;
use owo_colors::OwoColorize;

pub fn print_user(u: &User) {
    // TODO: Refactor these printlns to premade macros globally (ie header! text! long_text! etc)
    println!(
        "{}\n{}",
        u.display_name.to_string().bold().yellow(),
        "-".repeat(40)
    );
    println!("{:<12}: {}", "ID".blue(), u.id.to_string());
    println!("{:<12}: {}", "Slack ID".blue(), u.slack_id.to_string());

    println!("\n{}", "Avatar URL:".bold().cyan());
    println!("{}", u.avatar);

    println!("\n{}", "Project IDs:".bold().cyan());
    if u.project_ids.is_empty() {
        println!("- None -");
    } else {
        for id in &u.project_ids {
            println!("- {}", id);
        }
    }

    println!("\n{}", "Statistics:".bold().cyan());
    println!("{:<12}: {}", "Vote Count".blue(), u.vote_count.to_string());
    println!("{:<12}: {}", "Like Count".blue(), u.like_count.to_string());
    println!(
        "{:<12}: {}",
        "Devlog Seconds Total".blue(),
        u.devlog_seconds_total.to_string()
    );
    println!(
        "{:<12}: {}",
        "Devlog Seconds Today".blue(),
        u.devlog_seconds_today.to_string()
    );
    println!("{:<12}: {}", "Cookies".blue(), u.cookies.to_string());
}
