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

use serde::Deserialize;

fn zero_if_null<'de, D>(deserializer: D) -> Result<u32, D::Error>
// Sets duration to 0 if it is null because aparently it can be null.
where
    D: serde::Deserializer<'de>,
{
    Option::<u32>::deserialize(deserializer).map(|v| v.unwrap_or(0))
}

fn fix_displayname<'de, D>(deserializer: D) -> Result<String, D::Error>
// fixes a wierd edge case where a user has a null display name
where
    D: serde::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer).map(|v| v.unwrap_or("User".to_string()))
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub slack_id: String,
    #[serde(deserialize_with = "fix_displayname")]
    pub display_name: String,
    pub avatar: String,
    pub project_ids: Vec<u32>,
    pub vote_count: u32,
    pub like_count: u32,
    pub devlog_seconds_total: u32,
    pub devlog_seconds_today: u32,
    #[serde(deserialize_with = "zero_if_null")]
    pub cookies: u32,
}
