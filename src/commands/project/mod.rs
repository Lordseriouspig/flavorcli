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

pub mod devlog;
pub mod get;
pub mod list;

#[derive(Debug, Args)]
pub struct ProjectsCommand {
    // Defines project subcommand (level 2)
    #[clap(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ProjectSubcommand {
    // Defines project commands (level 3)
    /// List projects
    List(list::ProjectList),
    /// Get a specific project by its ID
    Get(get::ProjectGet),
    /// Commands that allow you to view devlogs for a project
    #[clap(alias = "devlogs")]
    Devlog(devlog::ProjectDevlogs),
}
