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

mod commands;
mod helpers;
mod models;

use clap::{Parser};
use commands::FlavorArgs;
use log::{error,trace};

#[tokio::main]
async fn main() {
    let args = FlavorArgs::parse();

    let mut builder = colog::basic_builder();
    builder
        .filter_level(args.verbosity.into())
        .init();

    match args.entity_type {
        commands::Command::Auth(auth_cmd) => {
            match auth_cmd.command {
                commands::auth::AuthSubcommand::Set(set_cmd) => {
                    if let Err(e) = set_cmd.execute().await {
                        error!("Failed to set authentication token: {}", e);
                        trace!("{:#?}", e);
                        std::process::exit(1);
                    }
                }
                commands::auth::AuthSubcommand::Delete(delete_cmd) => {
                    if let Err(e) = delete_cmd.execute().await {
                        error!("Failed to delete authentication token: {}", e);
                        trace!("{:#?}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        commands::Command::Project(project_cmd) => {
            match project_cmd.command {
                commands::project::ProjectSubcommand::Get(get_cmd) => {
                    if let Err(e) = get_cmd.execute().await {
                        error!("Failed to get project: {}", e);
                        trace!("{:#?}", e);
                        std::process::exit(1);
                    }
                }
                commands::project::ProjectSubcommand::List(list_cmd) => {
                    if let Err(e) = list_cmd.execute().await {
                        error!("Failed to list projects: {}", e);
                        trace!("{:#?}", e);
                        std::process::exit(1);
                    }
                }
                commands::project::ProjectSubcommand::Devlog(devlog_cmd) => {
                    match devlog_cmd.command {
                        commands::project::devlog::ProjectDevlogSubcommand::Get(get_cmd) => {
                            if let Err(e) = get_cmd.execute().await {
                                error!("Failed to get devlog: {}", e);
                                trace!("{:#?}", e);
                                std::process::exit(1);
                            }
                        }
                        commands::project::devlog::ProjectDevlogSubcommand::List(list_cmd) => {
                            if let Err(e) = list_cmd.execute().await {
                                error!("Failed to list devlogs: {}", e);
                                trace!("{:#?}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
        }
        commands::Command::Store(store_cmd) => {
            match store_cmd.command {
                commands::store::StoreSubcommand::Get(get_cmd) => {
                    if let Err(e) = get_cmd.execute().await {
                        error!("Failed to get store item: {}", e);
                        trace!("{:#?}", e);
                        std::process::exit(1);
                    }
                }
                commands::store::StoreSubcommand::List(list_cmd) => {
                    if let Err(e) = list_cmd.execute().await {
                        error!("Failed to list store items: {}", e);
                        trace!("{:#?}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        commands::Command::User(user_cmd) => {
            match user_cmd.command {
                commands::user::UserSubcommand::Get(get_cmd) => {
                    //TODO: Get Command
                }
                commands::user::UserSubcommand::List(list_cmd) => {
                    //TODO: List Command
                }
            }
        }
    }
}