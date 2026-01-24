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

Project list allows you to view a list of all projects and optionally search them.

## Usage
```bash
flavor project list [OPTIONS]
```

## Options
-  **``-p, --page <PAGE>``:**      Page number for pagination. Defaults to 1
-   **``   --query <QUERY>``:**    Query string to filter projects
-    **``  --json``:**             Returns data as raw JSON
-    **`` --fields <FIELDS>``:**  Fields to output in the table (advanced) [default: id,title,description,updated-at] [possible values: id, title, description, ship-status, repo-url, demo-url, readme-url, created-at, updated-at, devlog-ids]
-  **``-v, --verbose...``:**       Increase logging verbosity
-  **``-q, --quiet...``:**         Decrease logging verbosity
-  **``-h, --help``:**             Print help

## Examples
### List all projects (first page):
```bash
flavor project list
```
**Returns** (concatenated)
```
╭──────┬────────────────────────────────┬────────────────────────────────────────────────────┬──────────────────╮
│ ID   ┆ Title                          ┆ Description                                        ┆ Updated          │
╞══════╪════════════════════════════════╪════════════════════════════════════════════════════╪══════════════════╡
│ 3956 ┆ Tritonsspace's First Project   ┆ This is my first project on Flavortown. I'm exc... ┆ 2026-01-04 01:35 │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 6258 ┆ Browser Homepage in style o... ┆ A homepage for a browser with various customiza... ┆ 2026-01-08 10:19 │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 5384 ┆ nadipena murali's First Pro... ┆ This is my first project on Flavortown. I'm exc... ┆ 2026-01-07 14:11 │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 6408 ┆ Chef Robin's First Project     ┆ This is my first project on Flavortown. I'm exc... ┆ 2026-01-08 18:18 │
╰──────┴────────────────────────────────┴────────────────────────────────────────────────────┴──────────────────╯
Page 1/68 | Total results: 6795 | Next page: 2
```

### Search for projects
```bash
flavor project list --query "flavorcli"
```
**Returns**
```
Search result(s) for query 'flavorcli'
╭─────┬───────────┬────────────────────────────────────────────────────┬──────────────────╮
│ ID  ┆ Title     ┆ Description                                        ┆ Updated          │
╞═════╪═══════════╪════════════════════════════════════════════════════╪══════════════════╡
│ 333 ┆ FlavorCLI ┆ FlavorCLI is a feature-rich command line interf... ┆ 2026-01-14 12:18 │
╰─────┴───────────┴────────────────────────────────────────────────────┴──────────────────╯
Page 1/1 | Total results: 1
```

### List projects with custom columns
```bash
flavor project list --fields id,title,ship-status,created-at
```
**Returns** (concatenated)
```
╭──────┬────────────────────────────────┬───────────┬──────────────────╮
│ ID   ┆ Title                          ┆ Status    ┆ Created          │
╞══════╪════════════════════════════════╪═══════════╪══════════════════╡
│ 3956 ┆ Tritonsspace's First Project   ┆ draft     ┆ 2026-01-04 01:35 │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 6258 ┆ Browser Homepage in style o... ┆ draft     ┆ 2026-01-08 10:19 │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 3229 ┆ Fun Decision maker             ┆ submitted ┆ 2025-12-30 23:44 │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 6408 ┆ Chef Robin's First Project     ┆ draft     ┆ 2026-01-08 18:18 │
╰──────┴────────────────────────────────┴───────────┴──────────────────╯
Page 1/68 | Total results: 6795 | Next page: 2
```

### List projects in json
```bash
flavor project list --json
```
**Returns** (concatenated)
```json
{"projects":[{"id":3956,"title":"Tritonsspace's First Project","description":"This is my first project on Flavortown. I'm excited to share my progress!","ship_status":"draft","repo_url":null,"demo_url":null,"readme_url":null,"created_at":"2026-01-03T15:35:15.517Z","updated_at":"2026-01-03T15:35:18.155Z","devlog_ids":[5747]},{"id":6408,"title":"Chef Robin's First Project","description":"This is my first project on Flavortown. I'm excited to share my progress!","ship_status":"draft","repo_url":null,"demo_url":null,"readme_url":null,"created_at":"2026-01-08T08:18:41.087Z","updated_at":"2026-01-08T08:18:43.214Z","devlog_ids":[8683]}],"pagination":{"current_page":1,"total_pages":68,"total_count":6795,"next_page":2}}
```