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
use log::{error};
use commands::FlavorArgs;
use anyhow::{Context,Result};
use sentry::{start_session,end_session};
use sentry_anyhow::capture_anyhow;
use sentry_log::{SentryLogger,LogFilter};
use env_logger;


#[tokio::main]
async fn main() {
    let args: FlavorArgs = FlavorArgs::parse();
    let mut builder = env_logger::Builder::new();
    builder
        .format(colog::formatter(colog::format::DefaultCologStyle))
        .filter_level(args.verbosity.into());

    let logger = SentryLogger::with_dest(builder.build()).filter(|md| match md.level() {
    _ => LogFilter::Breadcrumb,
    });
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let _guard = sentry::init(("https://0e97a5a43a3fbf1c698d56f77aac426f@o4509766495043584.ingest.de.sentry.io/4510661862031440", sentry::ClientOptions {
        release: sentry::release_name!(),
        send_default_pii: true,
        debug: true,
        ..Default::default()
    }));

    start_session();
    if let Err(err) = run(args).await {
        error!("Error: {:?}", err);
        capture_anyhow(&err);
    }
    end_session();
}

async fn run(args: FlavorArgs) -> Result<()> {
    match args.entity_type {
        commands::Command::Auth(auth_cmd) => {
            match auth_cmd.command {
                commands::auth::AuthSubcommand::Set(set_cmd) => {
                    set_cmd.execute()
                    .await
                    .context("Failed to set authentication token")?;
                }
                commands::auth::AuthSubcommand::Delete(delete_cmd) => {
                    delete_cmd.execute()
                    .await
                    .context("Failed to delete authentication token")?;
                }
            }
        }
        commands::Command::Project(project_cmd) => {
            match project_cmd.command {
                commands::project::ProjectSubcommand::Get(get_cmd) => {
                    get_cmd.execute()
                    .await
                    .context("Failed to get project")?;
                }
                commands::project::ProjectSubcommand::List(list_cmd) => {
                    list_cmd.execute()
                    .await
                    .context("Failed to list projects")?;
                }
                commands::project::ProjectSubcommand::Devlog(devlog_cmd) => {
                    match devlog_cmd.command {
                        commands::project::devlog::ProjectDevlogSubcommand::Get(get_cmd) => {
                            get_cmd.execute()
                            .await
                            .context("Failed to get devlog")?;
                        }
                        commands::project::devlog::ProjectDevlogSubcommand::List(list_cmd) => {
                            list_cmd.execute()
                            .await
                            .context("Failed to list devlogs")?;
                        }
                    }
                }
            }
        }
        commands::Command::Store(store_cmd) => {
            match store_cmd.command {
                commands::store::StoreSubcommand::Get(get_cmd) => {
                    get_cmd.execute()
                    .await
                    .context("Failed to get store item")?;
                }
                commands::store::StoreSubcommand::List(list_cmd) => {
                    list_cmd.execute()
                    .await
                    .context("Failed to list store items")?;
                }
            }
        }
        commands::Command::User(user_cmd) => {
            match user_cmd.command {
                commands::user::UserSubcommand::Get(get_cmd) => {
                    get_cmd.execute()
                    .await
                    .context("Failed to get user")?;
                }
                commands::user::UserSubcommand::List(list_cmd) => {
                    list_cmd.execute()
                    .await
                    .context("Failed to list users")?;
                }
            }
        }
    }

    Ok(())
}