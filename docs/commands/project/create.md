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

The project create command allows you to create a new project.

## Usage
```bash
flavor project create [FLAGS] --title <TITLE> --description <DESCRIPTION>
```

### Flags
-  **`--title <TITLE>`:**              The title of the new project
-  **`--description <DESCRIPTION>`:**  The description of the new project
-  **`--repo-url <REPO_URL>`:**        A link to the project's repository
-  **`--demo-url <DEMO_URL>`:**        A link to the project's demo
-  **`--readme-url <README_URL>`:**    A link to the project's readme
-  **``-v, --verbose...``:**                 Increase logging verbosity
-  **`--json`:**                       Returns data as raw JSON
-  **``-q, --quiet...``:**                   Decrease logging verbosity
-  **``-h, --help``:**                       Print help

## Examples
Create a new project with a title and description:
```bash
flavor project create --title "Cool project" --description "wawawa"
```
**Returns**
```
Cool project
----------------------------------------
ID          : 8240
Status      : draft
Created     : 2026-01-14 19:00:55
Updated     : 2026-01-14 19:00:55

Description
wawawa

Links:
Repo        : -
Demo        : -
Readme      : -

Devlog IDs:
- None -
```

Create a project and return it as json
```bash
flavor project create --title "Cool project" --description "wawawa" --json
```
**Returns**
```json
{"id":8240,"title":"Cool project","description":"wawawa","ship_status":"draft","repo_url":null,"demo_url":null,"readme_url":null,"created_at":"2026-01-14T09:00:55.211Z","updated_at":"2026-01-14T09:00:55.211Z","devlog_ids":[]}
```