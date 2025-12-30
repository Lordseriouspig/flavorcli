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

use clap::{Args, Subcommand};

pub mod get;
pub mod list;

#[derive(Debug, Args)]
pub struct ProjectDevlogs {
    // Defines devlog subcommand (level 3)
    #[clap(subcommand)]
    pub command: ProjectDevlogSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ProjectDevlogSubcommand {
    // Defines devlog commands (level 4)
    /// List devlogs for a project
    List(list::ProjectDevlogList),

    /// Get a specific devlog by its ID
    Get(get::ProjectDevlogGet),
}
