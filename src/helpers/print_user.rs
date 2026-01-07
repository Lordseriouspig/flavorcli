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
use crate::{title, heading, field, long_text, list};

fn format_duration(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

pub fn print_user(u: &User) {
    title!(u.display_name);
    field!("ID", u.id);
    field!("Slack ID", u.slack_id);
    long_text!("Avatar URL", &u.avatar);

    heading!("Project IDs:");
    if u.project_ids.is_empty() {
        println!("- None -");
    } else {
        list!(&u.project_ids);
    }

    heading!("Statistics:");
    field!("Vote Count", u.vote_count);
    field!("Like Count", u.like_count);
    field!("Devlog Time Total", format_duration(u.devlog_seconds_total));
    field!("Devlog Time Today", format_duration(u.devlog_seconds_today));
    field!("Cookies", u.cookies);
}
