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

The project commands allow you to view and manage projects and devlogs on Flavortown.

## Usage
```bash
flavor.exe project [FLAGS] <COMMAND>
```

### Commands
-  **[`list`](./list.md):**    List projects
-  **[`get`](./get.md):**     Get a specific project by its ID
-  **[`devlog`](./devlog/README.md):**  Commands that allow you to view devlogs for a project
-  **[`create`](./create.md):**  Create a new project
-  **[`update`](./update.md):**  Update an existing project
-  **`help`:**    Print this message or the help of the given subcommand(s)

### Flags
-  **`-v, --verbose...`:**  Increase logging verbosity
-  **`-q, --quiet...`:**    Decrease logging verbosity
-  **`-h, --help`:**        Print help