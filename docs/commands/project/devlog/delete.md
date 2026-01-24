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

Devlog Delete allows you to delete an already existing devlog by its ID from a specific project.

## Usage
```bash
flavor project devlog delete [OPTIONS] <PROJECT_ID> <DEVLOG_ID>
```

### Arguments
-  **`<PROJECT_ID>`:**  The id of the project to delete a devlog from
-  **`<DEVLOG_ID>`:**   The id of the devlog to delete

### Options
-  **`-v, --verbose...`:**  Increase logging verbosity
-  **`-q, --quiet...`:**    Decrease logging verbosity
-  **`-h, --help`:**        Print help

## Examples
### Delete a devlog by its ID from a specific project
```bash
flavor project devlog delete 333 15142
```
**Returns**
```
Deleted devlog successfully.
```