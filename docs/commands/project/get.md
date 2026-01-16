<!--
 Copyright (C) 2026 Lordseriouspig
 
 This file is part of flavorcli.
 
 flavorcli is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 flavorcli is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with flavorcli.  If not, see <https://www.gnu.org/licenses/>.
-->

Project get allows you to get information about a specific project by its ID.

## Usage
```bash
flavor project get [FLAGS] <PROJECT_ID>
```

### Options
-  **``<PROJECT_ID>``:**  The project ID to retrieve
{% hint style="info" %}
You can get Project IDs from the `flavor project list` command, or from the `flavor user get` command, where you can see all projects associated with a user. The project's URL on Flavortown also contains the project ID.
{% endhint %}

### Flags
-  **``    --json    ``:**    Returns data as raw JSON
-  **``-r, --resolve``**     Resolves and displays devlogs in a project (May take longer)
-  **``-v, --verbose...``**  Increase logging verbosity
-  **``-q, --quiet...``**    Decrease logging verbosity
-  **``-h, --help``**        Print help

## Examples
### Get a project by its ID
```bash
flavor project get 333
```
**Returns** (concatenated)
```
FlavorCLI
----------------------------------------
ID          : 333
Status      : draft
Created     : 2025-12-20 13:28:05
Updated     : 2026-01-14 12:18:12

Description
FlavorCLI is a feature-rich command line interface for Flavortown. When
using FlavorCLI, you will be able to create devlogs, ship your projects,
view the shop, and more, right from your command line.

Links:
Repo        : https://github.com/lordseriouspig/flavorcli
Demo        : https://github.com/Lordseriouspig/flavorcli/releases/latest
Readme      : https://raw.githubusercontent.com/Lordseriouspig/flavorcli/main/README.MD

Devlog IDs:
- 11564
- 11107
- 10699
- 8725
```

### Get a project and return it as json
```bash
flavor project get 333 --json
```
**Returns**
```json
{"id":333,"title":"FlavorCLI","description":"FlavorCLI is a feature-rich command line interface for Flavortown. When using FlavorCLI, you will be able to create devlogs, ship your projects, view the shop, and more, right from your command line.","ship_status":"draft","repo_url":"https://github.com/lordseriouspig/flavorcli","demo_url":"https://github.com/Lordseriouspig/flavorcli/releases/latest","readme_url":"https://raw.githubusercontent.com/Lordseriouspig/flavorcli/main/README.MD","created_at":"2025-12-20T03:28:05.411Z","updated_at":"2026-01-14T02:18:12.224Z","devlog_ids":[11564,11107,10699,8725,6538,6265,5636,5624,5343,5025,5015,4727,4359,4250]}
```