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

Devlog Get allows you to get information about a specific devlog by it's, and optionally it's project's, ID.

## Usage

```bash
flavor project devlog get [FLAGS] <DEVLOG_ID>
```

### Options
- `<DEVLOG_ID>`  The devlog ID to retrieve
{% hint style="info" %}
You can get Devlog IDs from the `flavor project devlog list` command, or from the `flavor project get` command, where you can see all devlogs associated with a project.
{% endhint %}

### Flags
-  **`-p, --project-id <PROJECT_ID>`**:  The project ID the devlog belongs to. Will search for this devlog across all projects if not provided
-  **`    --json`**:                     Returns data as raw JSON
-  **`-s, --short`**:                    Omits the devlog's metadata
-  **`-v, --verbose...`**:               Increase logging verbosity
-  **`-q, --quiet...`**:                 Decrease logging verbosity
-  **`-h, --help`**:                     Print help

## Examples
### Get a devlog by its ID
```bash
flavor project devlog get 11564
```
**Returns**
```
Devlog #11564
----------------------------------------
Comments    : 0
Time        : 06:21:37
Likes       : 0
Created     : 2026-01-14 12:18:11
Updated     : 2026-01-14 12:18:12

Body
Those "quick minor features" were defiantly "quick and minor"...
Yeah so they were much more tedious then I thought they would be haha,
I've been working on adding some more customization flags to some of
the commands. These are namely custom fields! Yes, you can now customise
what fields will show up in the table. You can also resolve a user's
projects, and if you want a really long output, each of those project's
devlogs. There's also sorting for the shop!
Anyway, I'm now working on adding documentation to GitBook, and once
that's finished, I'll be working on releasing the next release.
```


### Get json of a devlog by its ID
```bash
flavor project devlog get 11564 --json
```

**Returns**
```json
{"id":11564,"body":"Those \"quick minor features\" were defiantly \"quick and minor\"...\r\nYeah so they were much more tedious then I thought they would be haha, I've been working on adding some more customization flags to some of the commands. These are namely custom fields! Yes, you can now customise what fields will show up in the table. You can also resolve a user's projects, and if you want a really long output, each of those project's devlogs. There's also sorting for the shop!\r\nAnyway, I'm now working on adding documentation to GitBook, and once that's finished, I'll be working on releasing the next release.","comments_count":0,"duration_seconds":22897,"likes_count":0,"scrapbook_url":null,"created_at":"2026-01-14T02:18:11.393Z","updated_at":"2026-01-14T02:18:12.221Z"}
```