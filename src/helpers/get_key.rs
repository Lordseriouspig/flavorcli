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

use anyhow::Context;
use keyring::Entry;
use log::debug;

use crate::models::authdata::AuthData;

pub fn get_key() -> anyhow::Result<AuthData> {
    debug!("Retrieving authentication key from credential store");
    let entry =
        Entry::new("flavorcli", "auth_token").context("Failed to access credential store")?;

    let json = entry
        .get_password()
        .context("No authentication token found. Please set your token using `ft auth set`")?;

    let auth: AuthData =
        serde_json::from_str(&json).context("Failed to parse authentication data")?;

    debug!("Successfully retrieved and parsed authentication key");

    Ok(auth)
}
