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

use clap::{Parser, Subcommand};

pub mod auth;
pub mod project;
pub mod store;
pub mod user;

#[derive(Parser)]
#[clap(name = "ft", author, version, about)]
#[derive(Debug)]
pub struct FlavorArgs {
    // Defines the top level command
    #[clap(subcommand)]
    pub entity_type: Command,
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]

pub enum Command {
    // Defines each subcommand (level 2)
    /// Commands that allow you to manage your authorization to Flavortown.
    Auth(auth::AuthCommand),

    /// Commands that allow you to view and search projects and devlogs.
    #[clap(alias = "projects")]
    Project(project::ProjectsCommand),

    /// Commands that allow you to browse the Flavortown store.
    Store(store::StoreCommand),

    /// Commands that allow you to view information about users.
    #[clap(alias = "users")]
    User(user::UsersCommand),
}
