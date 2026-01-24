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

Devlog Create allows you to create a new devlog for a specific project.

## Usage

```bash
flavor project devlog create [FLAGS] --body <BODY> <PROJECT_ID>
```

### Options
- **`<PROJECT_ID>`:**  The id of the project to add a devlog to

{% hint style="info" %}
You can get Project IDs from the `flavor project list` command, or from the `flavor user get` command, where you can see all projects associated with a user. The project's URL on Flavortown also contains the project ID.
{% endhint %}


### Flags
-   **`   --body <BODY>`:**              The body of the new devlog
-   **`   --attachment <ATTACHMENT>`:**  The paths to the attachments to add to the devlog
-   **`   --json`:**                     Returns data as raw JSON
-  **`-v, --verbose...`:**               Increase logging verbosity
-  **`-q, --quiet...`:**                 Decrease logging verbosity
-  **`-h, --help`:**                     Print help

## Examples
### Create a new devlog for a specific project
```bash
cargo run -- project devlog create 333 --body "Hi guys! I'm really excited to be presenting the newest feature of FlavorCLI. And this devlog will be a bit different, as today, I am creating this devlog through FlavorCLI! Yes, that's right, you can now create devlogs through FlavorCLI. This should hopefully mean I'll be able to ship v1.0.0 very soon! You can also upload and delete devlogs through FlavorCLI too!" --attachment "C:\Users\lachl\Downloads\Screenshot 2026-01-24 154304.png"
```
**Returns**
```
Devlog #15142
----------------------------------------
Comments    : 0
Time        : 04:54:41
Likes       : 0
Created     : 2026-01-24 15:48:10
Updated     : 2026-01-24 15:48:10

Body
Hi guys! I'm really excited to be presenting the newest feature of
FlavorCLI. And this devlog will be a bit different, as today, I am
creating this devlog through FlavorCLI! Yes, that's right, you can now
create devlogs through FlavorCLI. This should hopefully mean I'll be
able to ship v1.0.0 very soon! You can also upload and delete devlogs
through FlavorCLI too!

Media:
Attachment #1: https://flavortown.hackclub.com/rails/active_storage/blobs/proxy/eyJfcmFpbHMiOnsiZGF0YSI6ODEwOTksInB1ciI6ImJsb2JfaWQifX0=--aed4ea961044385f3397ff85efd985c80eaaa23e/Screenshot%202026-01-24%20154304.png (image/png)
```