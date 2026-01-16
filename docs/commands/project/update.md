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

Allows you to update an existing project that you own
## Usage
```bash
flavor project update [OPTIONS] <PROJECT_ID>
```

### Options
-  **``<PROJECT_ID>``:**  The project ID to update

### Flags
-  **``--title <TITLE>``:**              The title of the new project
-  **``--description <DESCRIPTION>``:**  The description of the new project
-  **``--repo-url <REPO_URL>``:**        A link to the project's repository
-  **``--demo-url <DEMO_URL>``:**        A link to the project's demo
-  **``--readme-url <README_URL>``:**    A link to the project's readme
-  **``-v, --verbose...``:**                 Increase logging verbosity
-  **``--json``:**                       Returns data as raw JSON
-  **``-q, --quiet...``:**                   Decrease logging verbosity
-  **``--put``:**                        Overrides the resource to be exactly what you specify (PUT instead of PATCH)
-  **``-h, --help``:**                       Print help

## Examples
### Update a project's title and description:
```bash
project update 8240 --title "even cooler project" --description "wawawawawawawawawawawawawawawawawawawa"
```

**Returns**
```
even cooler project
----------------------------------------
ID          : 8240
Status      : draft
Created     : 2026-01-14 19:00:55
Updated     : 2026-01-14 19:38:45

Description
wawawawawawawawawawawawawawawawawawawa

Links:
Repo        : -
Demo        : -
Readme      : -

Devlog IDs:
- None -
```

### Add links to a project
```bash
project update 8240 --repo-url "https://github.com/hackclub/flavortown" --demo-url "https://example.com"
```
**Returns**
```
even cooler project
----------------------------------------
ID          : 8240
Status      : draft
Created     : 2026-01-14 19:00:55
Updated     : 2026-01-14 19:50:14

Description
wawawawawawawawawawawawawawawawawawawa

Links:
Repo        : https://github.com/hackclub/flavortown
Demo        : https://example.com
Readme      : -

Devlog IDs:
- None -
```