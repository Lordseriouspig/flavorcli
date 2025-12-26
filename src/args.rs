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
    pub entity_type: Command,
}

#[derive(Debug, Subcommand)]

pub enum Command {
    // Defines each subcommand (level 2)
    /// Commands that allow you to manage your authorization to Flavortown.
    Auth(AuthCommand),

    /// Commmands that allow you to view and search projects and devlogs.
    #[clap(alias = "projects")]
    Project(ProjectsCommand),

    /// Commands that allow you to browse the Flavortown store.
    Store(StoreCommand),

    /// Commands that allow you to view informations about users.
    #[clap(alias = "users")]
    User(UsersCommand),
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
    Set(TokenSet),

    /// Delete your authentication token
    Delete(TokenDelete),
}

#[derive(Debug, Args)]
pub struct TokenSet {
    // Defines set command (level 3)
    /// Your Flavortown authentication token
    pub token: String,
}

#[derive(Debug, Args)]
pub struct TokenDelete; // Defines deletetoken command (level 3)

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
    List(ProjectList),
    /// Get a specific project by its ID
    Get(ProjectGet),
    /// Commands that allow you to view devlogs for a project
    Devlogs(ProjectDevlogs),
}

#[derive(Debug, Args)]
pub struct ProjectList {
    // Defines createproject command (level 3)
    /// Page number for pagination
    #[clap(long, short)]
    pub page: Option<u32>,

    /// Query string to filter projects
    #[clap(long, short)]
    pub query: Option<String>,
}

#[derive(Debug, Args)]
pub struct ProjectGet {
    // Defines getproject command (level 3)
    pub project_id: u64,
}

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
    List(ProjectDevlogList),

    /// Get a specific devlog by its ID
    Get(ProjectDevlogGet),
}

#[derive(Debug, Args)]
pub struct ProjectDevlogList {
    // Defines list devlogs command (level 4)
    /// The project ID to list devlogs for
    pub project_id: u64,

    /// Page number for pagination
    #[clap(long, short)]
    pub page: Option<u32>,
}

#[derive(Debug, Args)]
pub struct ProjectDevlogGet {
    // Defines get devlog command (level 4)
    /// The project ID the devlog belongs to
    pub project_id: u64,
    /// The devlog ID to retrieve
    pub devlog_id: u64,
}

#[derive(Debug, Args)]
pub struct StoreCommand {
    // Defines store subcommand (level 2)
    #[clap(subcommand)]
    pub command: StoreSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum StoreSubcommand {
    // Defines store commands (level 3)
    /// List store items
    List(StoreList),
    /// Get a specific store item by its ID
    Get(StoreGet),
}

#[derive(Debug, Args)]
pub struct StoreList {
    // Defines list store items command (level 3)
}

#[derive(Debug, Args)]
pub struct StoreGet {
    // Defines get store item command (level 3)
    /// The store item ID to retrieve
    #[clap(long, short)]
    pub item_id: u64,
}

#[derive(Debug, Args)]
pub struct UsersCommand {
    // Defines user subcommand (level 2)
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    // Defines user commands (level 3)
    /// Get a list of users
    List(UserList),
    /// Get user information by ID
    Get(UserGet),
}

#[derive(Debug, Args)]
pub struct UserList {
    // Defines list users command (level 3)
    /// Page number for pagination
    #[clap(long, short)]
    pub page: Option<u32>,
}

#[derive(Debug, Args)]
pub struct UserGet {
    // Defines get user command (level 3)
    /// The user ID to retrieve
    pub user_id: u64,
}
