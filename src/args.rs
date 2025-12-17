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

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "ft", author, version, about)]

pub struct FlavorArgs {
    // Defines the top level command
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]

pub enum EntityType {
    // Defines each subcommand (level 2)
    /// Commands that allow you to manage your authorization to Flavortown.
    Auth(AuthCommand),

    /// Devlog Commands
    Devlog(DevlogCommand),

    /// Project Commands
    Project(ProjectCommand),
}

#[derive(Debug, Args)]

pub struct AuthCommand {
    // Defines auth subcommand (level 2)
    #[clap(subcommand)]
    pub command: AuthSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AuthSubcommand {
    // Defines auth commands (level 3)
    /// Set your authentication token
    Set(Set),

    /// Delete your authentication token
    Delete(DeleteToken),
}

#[derive(Debug, Args)]
pub struct Set {
    // Defines set command (level 3)
    /// Your Flavortown authentication token
    pub token: String,
}

#[derive(Debug, Args)]
pub struct DeleteToken; // Defines deletetoken command (level 3)

#[derive(Debug, Args)]
pub struct DevlogCommand {
    // Defines devlog subcommand (level 2)
    #[clap(subcommand)]
    pub command: DevlogSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DevlogSubcommand {
    // Defines devlog commands (level 3)
    /// Create a new devlog entry
    Create(CreateDevlog),
}

#[derive(Debug, Args)]
pub struct CreateDevlog {
    // Defines createdevlog command (level 3)
    /// Title of the devlog entry
    pub title: String,

    /// Content of the devlog entry
    pub content: String,
}

#[derive(Debug, Args)]
pub struct ProjectCommand {
    // Defines project subcommand (level 2)
    #[clap(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ProjectSubcommand {
    // Defines project commands (level 3)
    /// Create a new project
    Create(CreateProject),
}

#[derive(Debug, Args)]
pub struct CreateProject {
    // Defines createproject command (level 3)
    /// Name of the project
    pub name: String,

    /// Description of the project
    pub description: String,
}
