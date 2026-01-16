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

The Devlog list command allows you to list devlogs for a specific project or all projects.

## Usage
```bash
flavor project devlog list [Flags] [PROJECT_ID]
```

### Options
-  **`[PROJECT_ID]`:**  The project ID to list devlogs for. Will list ALL devlogs if not provided

### Flags
-  **`-p, --page <PAGE>`:**      Page number for pagination. Defaults to 1
-  **`    --json       `:**      Returns data as raw JSON
-  **`    --fields <FIELDS>`:**  Fields to output in the table (advanced) [default: id,body,duration,likes-count,comments-count,updated-at] [possible values: id, body, comments-count, duration, likes-count, scrapbook-url, created-at, updated-at]
-  **`-v, --verbose...`:**       Increase logging verbosity
-  **`-q, --quiet...`:**         Decrease logging verbosity
-  **`-h, --help`:**             Print help

## Examples
### Get all devlogs of a project
```bash
flavor project devlog list 333
```
**Returns** (concatenated)
```
Devlogs for project: 'FlavorCLI'
╭───────┬────────────────────────────────────────────────────┬──────────┬───────┬──────────┬──────────────────╮
│ ID    ┆ Body                                               ┆ Duration ┆ Likes ┆ Comments ┆ Updated          │
╞═══════╪════════════════════════════════════════════════════╪══════════╪═══════╪══════════╪══════════════════╡
│ 11564 ┆ Those "quick minor features" were defiantly "qu... ┆ 06:21:37 ┆ 0     ┆ 0        ┆ 2026-01-14 12:18 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 11107 ┆ The new project create and update commands are ... ┆ 00:30:16 ┆ 0     ┆ 0        ┆ 2026-01-13 11:17 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 8725  ┆ I'm happy to announce version 0.2.0-beta.1!  In... ┆ 06:07:40 ┆ 0     ┆ 2        ┆ 2026-01-08 20:13 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 6538  ┆ I've done some boring stuff regarding releases ... ┆ 03:57:37 ┆ 1     ┆ 1        ┆ 2026-01-06 19:44 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 4359  ┆ I've written and finished the ``ft.exe auth set... ┆ 02:21:40 ┆ 0     ┆ 0        ┆ 2025-12-30 18:52 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 4250  ┆ I was having some issues with my devlog time no... ┆ 05:55:22 ┆ 0     ┆ 0        ┆ 2025-12-30 11:58 │
╰───────┴────────────────────────────────────────────────────┴──────────┴───────┴──────────┴──────────────────╯
Page 1/1 | Total results: 14
```

### Get all devlogs of a project with custom columns
```bash
flavor project devlog list 333 --fields id,body,duration
```
**Returns** (concatenated)
```
Devlogs for project: 'FlavorCLI'
╭───────┬────────────────────────────────────────────────────┬──────────╮
│ ID    ┆ Body                                               ┆ Duration │
╞═══════╪════════════════════════════════════════════════════╪══════════╡
│ 11564 ┆ Those "quick minor features" were defiantly "qu... ┆ 06:21:37 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ 11107 ┆ The new project create and update commands are ... ┆ 00:30:16 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ 8725  ┆ I'm happy to announce version 0.2.0-beta.1!  In... ┆ 06:07:40 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ 6538  ┆ I've done some boring stuff regarding releases ... ┆ 03:57:37 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ 4359  ┆ I've written and finished the ``ft.exe auth set... ┆ 02:21:40 │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ 4250  ┆ I was having some issues with my devlog time no... ┆ 05:55:22 │
╰───────┴────────────────────────────────────────────────────┴──────────╯
Page 1/1 | Total results: 14
```

### Get all devlogs of a project in json
```bash
flavor project devlog list 333 --json
```
**Returns** (concatenated)
```json
{"devlogs":[{"id":11564,"body":"Those \"quick minor features\" were defiantly \"quick and minor\"...\r\nYeah so they were much more tedious then I thought they would be haha, I've been working on adding some more customization flags to some of the commands. These are namely custom fields! Yes, you can now customise what fields will show up in the table. You can also resolve a user's projects, and if you want a really long output, each of those project's devlogs. There's also sorting for the shop!\r\nAnyway, I'm now working on adding documentation to GitBook, and once that's finished, I'll be working on releasing the next release.","comments_count":0,"duration_seconds":22897,"likes_count":0,"scrapbook_url":null,"created_at":"2026-01-14T02:18:11.393Z","updated_at":"2026-01-14T02:18:12.221Z","media":[{"url":"/rails/active_storage/blobs/proxy/eyJfcmFpbHMiOnsiZGF0YSI6NjMxNjEsInB1ciI6ImJsb2JfaWQifX0=--4d3eb44ce45ac2b8ccabc82a08190c84a38e62f4/pasted-2026-01-14T02-13-20-408Z.png","content_type":"image/png"},{"url":"/rails/active_storage/blobs/proxy/eyJfcmFpbHMiOnsiZGF0YSI6NjMxNjIsInB1ciI6ImJsb2JfaWQifX0=--4f8fde4ea82a6b2e69a0cd0838924a91ae99dbeb/pasted-2026-01-14T02-17-55-727Z.png","content_type":"image/png"}]},{"id":4250,"body":"I was having some issues with my devlog time not showing up, but that should be fixed now, so I can finally make a devlog.\r\nI'm working on making a command-line interface for Flavor Town. My goals are to integrate the Flavortown API directly first, and afterwards, add more complex logic. I'm aiming to make it feasible and easy to create devlogs and manage your Flavor Town projects right from your command line!\r\nSo far, I've made a quick Rust skeleton (yes I've decided to learn Rust for this) using Clap, based on the current API. Next steps will be to add a command to save your API key, integrate it with the Flavor Town API, and then work on some more complex logic after that.","comments_count":0,"duration_seconds":21322,"likes_count":0,"scrapbook_url":null,"created_at":"2025-12-30T01:58:05.782Z","updated_at":"2025-12-30T01:58:06.736Z","media":[{"url":"/rails/active_storage/blobs/proxy/eyJfcmFpbHMiOnsiZGF0YSI6MjYzMjgsInB1ciI6ImJsb2JfaWQifX0=--04f68e0cac86c9c8fc5a38c5898e7f918b0544d8/pasted-2025-12-30T01-57-55-956Z.png","content_type":"image/png"}]}],"pagination":{"current_page":1,"total_pages":1,"total_count":14,"next_page":null}}
```