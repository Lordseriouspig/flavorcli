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

The user list command allows you to get a list of every user.

## Usage
```bash
flavor user list [FLAGS]
```

### Flags
-  **``-p, --page <PAGE>``:**      Page number for pagination
-    **``  --json``:**             Returns data as raw JSON
-   **``   --fields <FIELDS>``:**  Fields to output in the table (advanced) [default: id,display-name,cookies] [possible values: id, slack-id, display-name, avatar, project-ids, cookies]
-  **``-v, --verbose...``:**       Increase logging verbosity
-  **``-q, --quiet...``:**         Decrease logging verbosity
-  **``-h, --help``:**             Print help

## Examples
### Get all users (page 1)
```bash
flavor user list
```
**Returns** (concatenated)
```
╭──────┬───────────────────────┬─────────╮
│ ID   ┆ Display Name          ┆ Cookies │
╞══════╪═══════════════════════╪═════════╡
│ 554  ┆ Puneet                ┆ 0       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 76   ┆ alex                  ┆ 0       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 1450 ┆ Lalith Satheesh       ┆ 0       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 2758 ┆ patrickioanc          ┆ 0       │
╰──────┴───────────────────────┴─────────╯
Page 1/57 | Total users: 5647 | Next page: 2
```

### Get a list of users with custom fields
```bash
flavor user list --fields id,display-name,slack-id,cookies
```
**Returns** (concatenated)
```
╭──────┬───────────────────────┬─────────────┬─────────╮
│ ID   ┆ Display Name          ┆ Slack ID    ┆ Cookies │
╞══════╪═══════════════════════╪═════════════╪═════════╡
│ 554  ┆ Puneet                ┆ U0925CUPLHZ ┆ 0       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 76   ┆ alex                  ┆ U08JTTQHQDB ┆ 0       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 1450 ┆ Lalith Satheesh       ┆ U09UM7SBE6S ┆ 0       │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 2758 ┆ patrickioanc          ┆ U0A6Y84HTM3 ┆ 0       │
╰──────┴───────────────────────┴─────────────┴─────────╯
```

### Get a list of users in json
```bash
flavor user list --json
```
**Returns** (concatenated)
```json
{"users":[{"id":554,"slack_id":"U0925CUPLHZ","display_name":"Puneet","avatar":"https://cachet.dunkirk.sh/users/U0925CUPLHZ/r","project_ids":[542],"cookies":null},{"id":76,"slack_id":"U08JTTQHQDB","display_name":"alex","avatar":"https://cachet.dunkirk.sh/users/U08JTTQHQDB/r","project_ids":[],"cookies":null},{"id":2758,"slack_id":"U0A6Y84HTM3","display_name":"patrickioanc","avatar":"https://cachet.dunkirk.sh/users/U0A6Y84HTM3/r","project_ids":[4398],"cookies":null}],"pagination":{"current_page":1,"total_pages":57,"total_count":5647,"next_page":2}}
```