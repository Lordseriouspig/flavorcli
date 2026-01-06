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

use clap::Args;
use keyring::Entry;
use log::{debug, info};

#[derive(Debug, Args)]
pub struct AuthDelete; // Defines delete command (level 3)

impl AuthDelete {
    pub async fn execute(&self) -> anyhow::Result<()> {
        debug!("Starting auth delete operation");
        let entry = Entry::new("flavorcli", "auth_token")?;
        debug!("Deleting credential from keyring");
        entry.delete_credential()?;
        info!("Authentication token deleted from keyring.");

        println!("Authentication token deleted successfully.");
        Ok(())
    }
}
