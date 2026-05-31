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

The user get command allows you to get information about a specific user by their ID.

## Usage
```bash
star user get [OPTIONS] [USER_ID]
```

### Arguments
-  **``[USER_ID]``:**  The user ID to retrieve. Defaults to you if not supplied

### Options
-    **``  --json``:**        Returns data as raw JSON
-  **``-r, --resolve...``:**  Resolves project IDs, or when specified twice, devlog ids as well (may be VERY long)
-  **``-v, --verbose...``:**  Increase logging verbosity
-  **``-q, --quiet...``:**    Decrease logging verbosity
-  **``-h, --help``:**        Print help

## Examples
### Get information about yourself
```bash
star user get
```
**Returns**
```
Lordseriouspig
----------------------------------------
ID          : 190
Slack ID    : U096NGLTF4K

Avatar URL
https://cachet.dunkirk.sh/users/U096NGLTF4K/r

Project IDs:
- 333
- 335
- 8240

Statistics:
Vote Count  : 1
Like Count  : 0
Devlog Time Total: 53:54:17
Devlog Time Today: 00:00:00
Cookies     : 47
```

### Get information about a specific user by ID
```bash
star user get 2
```
**Returns**
```
msw
----------------------------------------
ID          : 2
Slack ID    : U0C7B14Q3

Avatar URL
https://cachet.dunkirk.sh/users/U0C7B14Q3/r

Project IDs:
- 1

Statistics:
Vote Count  : 10
Like Count  : 1
Devlog Time Total: 04:21:33
Devlog Time Today: 00:00:00
Cookies     : 0
```

### Get information about a user and resolve their projects
```bash
star user get 2 --resolve
```
**Returns**
```
msw
----------------------------------------
ID          : 2
Slack ID    : U0C7B14Q3

Avatar URL
https://cachet.dunkirk.sh/users/U0C7B14Q3/r

Projects:
Stardance
----------------------------------------
ID          : 1
Status      : draft
Created     : 2025-12-06 10:23:41
Updated     : 2026-01-15 15:07:44

Description
yep! I'm logging this on the platform itself (am staff)

Links:
Repo        : https://github.com/hackclub/stardance
Demo        : https://stardance.hackclub.com
Readme      : https://raw.githubusercontent.com/hackclub/stardance/refs/heads/main/README.md

Devlog IDs:
- 5194
- 1


Statistics:
Vote Count  : 10
Like Count  : 1
Devlog Time Total: 04:21:33
Devlog Time Today: 00:00:00
Cookies     : 0
```

### Get information about a user and resolve their projects AND devlogs (can be very long)
```bash
star user get 2 --resolve --resolve
```
or
```bash
star user get 2 -rr
```
**Returns**
```
msw
----------------------------------------
ID          : 2
Slack ID    : U0C7B14Q3

Avatar URL
https://cachet.dunkirk.sh/users/U0C7B14Q3/r

Projects:
Stardance
----------------------------------------
ID          : 1
Status      : draft
Created     : 2025-12-06 10:23:41
Updated     : 2026-01-15 15:07:44

Description
yep! I'm logging this on the platform itself (am staff)

Links:
Repo        : https://github.com/hackclub/stardance
Demo        : https://stardance.hackclub.com
Readme      : https://raw.githubusercontent.com/hackclub/stardance/refs/heads/main/README.md

Devlogs:
Devlog #5194
----------------------------------------

Body
Check out https://stardance.hackclub.com/explore/extensions! we now
have a list of projects that use the stardance api ranked by the number
of total users. if you're building something stardance related you
should set your headers!

Devlog #1
----------------------------------------

Body
aggugugguauuauauaua



Statistics:
Vote Count  : 10
Like Count  : 1
Devlog Time Total: 04:21:33
Devlog Time Today: 00:00:00
Cookies     : 0
```

### Get information about a user in json
```bash
star user get 2 --json
```
**Returns**
```json
{"id":2,"slack_id":"U0C7B14Q3","display_name":"msw","avatar":"https://cachet.dunkirk.sh/users/U0C7B14Q3/r","project_ids":[1],"vote_count":10,"like_count":1,"devlog_seconds_total":15693,"devlog_seconds_today":0,"cookies":null}
```