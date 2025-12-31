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

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub ship_status: String,
    pub repo_url: String,
    pub demo_url: String,
    pub readme_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub devlog_ids: Vec<u64>,
}
