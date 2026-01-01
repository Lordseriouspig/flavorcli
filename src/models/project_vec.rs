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

use crate::models::project::Project;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_count: u32,
    pub next_page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectVec {
    pub projects: Vec<Project>,
    pub pagination: Pagination,
}