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
use crate::{field, heading, long_text, title};
use chrono::{DateTime, Local};
use owo_colors::OwoColorize;

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

pub fn print_devlog(d: &Devlog, short: bool) {
    title!(format!("Devlog #{}", d.id));
    if !short {
        field!("Comments", d.comments_count);
        field!("Time", format_duration(d.duration_seconds));
        field!("Likes", d.likes_count);
        field!("Created", format_time(&d.created_at));
        field!("Updated", format_time(&d.updated_at));
    }

    long_text!("Body", &d.body);

    if d.scrapbook_url.is_some() && !short {
        heading!("Scrapbook:");
        field!("URL", d.scrapbook_url.as_ref().unwrap());
    }

    if d.media.is_some() && !short {
        heading!("Media:");
        for (i, media) in d.media.as_ref().unwrap().iter().enumerate() {
            field!(format!("Attachment #{}", i + 1), format!("https://flavortown.hackclub.com{} ({})", media.url, media.content_type));
        }
    }
}
