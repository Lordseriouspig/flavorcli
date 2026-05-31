<!--
 Copyright (C) 2026 Lordseriouspig
 
 This file is part of starcli.
 
 starcli is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 starcli is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with starcli.  If not, see <https://www.gnu.org/licenses/>.
-->

Project get allows you to get information about a specific project by its ID.

## Usage
```bash
star project get [OPTIONS] <PROJECT_ID>
```

### Arguments
-  **``<PROJECT_ID>``:**  The project ID to retrieve
{% hint style="info" %}
You can get Project IDs from the `star project list` command, or from the `star user get` command, where you can see all projects associated with a user. The project's URL on Stardance also contains the project ID.
{% endhint %}

### Options
-  **``    --json    ``:**    Returns data as raw JSON
-  **``-r, --resolve``**     Resolves and displays devlogs in a project (May take longer)
-  **``-v, --verbose...``**  Increase logging verbosity
-  **``-q, --quiet...``**    Decrease logging verbosity
-  **``-h, --help``**        Print help

## Examples
### Get a project by its ID
```bash
star project get 333
```
**Returns** (concatenated)
```
StarCLI
----------------------------------------
ID          : 333
Status      : draft
Created     : 2025-12-20 13:28:05
Updated     : 2026-01-14 12:18:12

Description
StarCLI is a feature-rich command line interface for Stardance. When
using StarCLI, you will be able to create devlogs, ship your projects,
view the shop, and more, right from your command line.

Links:
Repo        : https://github.com/lordseriouspig/starcli
Demo        : https://github.com/Lordseriouspig/starcli/releases/latest
Readme      : https://raw.githubusercontent.com/Lordseriouspig/starcli/main/README.MD

Devlog IDs:
- 11564
- 11107
- 10699
- 8725
```

### Get a project and return it as json
```bash
star project get 333 --json
```
**Returns**
```json
{"id":333,"title":"StarCLI","description":"StarCLI is a feature-rich command line interface for Stardance. When using StarCLI, you will be able to create devlogs, ship your projects, view the shop, and more, right from your command line.","ship_status":"draft","repo_url":"https://github.com/lordseriouspig/starcli","demo_url":"https://github.com/Lordseriouspig/starcli/releases/latest","readme_url":"https://raw.githubusercontent.com/Lordseriouspig/starcli/main/README.MD","created_at":"2025-12-20T03:28:05.411Z","updated_at":"2026-01-14T02:18:12.224Z","devlog_ids":[11564,11107,10699,8725,6538,6265,5636,5624,5343,5025,5015,4727,4359,4250]}
```