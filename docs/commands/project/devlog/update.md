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

Devlog Update allows you to update an existing devlog for a specific project.
## Usage

```bash
flavor project devlog update [FLAGS] <PROJECT_ID> <DEVLOG_ID>
```
### Options
-  **`<PROJECT_ID>`:**  The id of the project to update a devlog in
-  **`<DEVLOG_ID>`:**   The id of the devlog to update

### Flags
-   **`   --body <BODY>`:**  The new body of the devlog
-   **`   --json`:**         Returns data as raw JSON
-    **`  --put`:**          Overrides the resource to be exactly what you specify (PUT instead of PATCH)
-  **`-v, --verbose...`:**   Increase logging verbosity
-  **`-q, --quiet...`:**     Decrease logging verbosity
-  **`-h, --help`:**         Print help

## Examples
### Update an existing devlog for a specific project
```bash
flavor project devlog update 333 15142 --body "Hi guys! I'm really excited to be presenting the newest feature of FlavorCLI. And this devlog will be a bit different, as today, I am creating this devlog through FlavorCLI! Yes, that's right, you can now create devlogs through FlavorCLI. This should hopefully mean I'll be able to ship v1.0.0 very soon! You can also upload and delete devlogs through FlavorCLI too! Infact, I have just used the update command to add this new text!"
```
**Returns**
```
Devlog #15142                                                                                                                                                                                                                                          
----------------------------------------
Comments    : 0
Time        : 04:54:41
Likes       : 0
Created     : 2026-01-24 15:48:10
Updated     : 2026-01-24 15:54:07

Body
Hi guys! I'm really excited to be presenting the newest feature of
FlavorCLI. And this devlog will be a bit different, as today, I am
creating this devlog through FlavorCLI! Yes, that's right, you can now
create devlogs through FlavorCLI. This should hopefully mean I'll be
able to ship v1.0.0 very soon! You can also upload and delete devlogs
through FlavorCLI too! Infact, I have just used the update command to
add an attachment of me writing this devlog from the CLI, and add this
new text!

Media:
Attachment #1: https://flavortown.hackclub.com/rails/active_storage/blobs/proxy/eyJfcmFpbHMiOnsiZGF0YSI6ODEwOTksInB1ciI6ImJsb2JfaWQifX0=--aed4ea961044385f3397ff85efd985c80eaaa23e/Screenshot%202026-01-24%20154304.png (image/png)
```