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

![FlavorCLI Banner](https://hc-cdn.hel1.your-objectstorage.com/s/v3/d80496dcf9688766_untitled__1_.png)
<!-- gotta love the crude attempt at a cross-platform note -->

> [!TIP]
> This page best viewed on [GitBook](https://flavorcli.gitbook.io/flavorcli)! We highly recommend reading the docs there, as some formatting does not work in raw markdown properly, and it provides for easier navigation.

Welcome to the FlavorCLI docs! Here you may find all the information you need to use FlavorCLI.
To get started, take a look at the **Table of Contents** on the left. There you may find guides on many commands and concepts.


## Quick Links
<table data-view="cards">
  <thead>
    <tr>
      <th></th>
      <th></th>
      <th data-hidden data-card-target data-type="content-ref"></th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>Installation</strong></td>
      <td>Install FlavorCLI on your system</td>
      <td><a href="./installation.md">Installation</a></td>
    </tr>
    <tr>
      <td><strong>Getting Started</strong></td>
      <td>Setup and basic usage</td>
      <td><a href="./getting_started.md">Getting Started</a></td>
    </tr>
    <tr>
      <td><strong>Add to PATH</strong></td>
      <td>Make FlavorCLI available everywhere</td>
      <td><a href="./add_to_path.md">Add to PATH</a></td>
    </tr>
    <tr>
      <td><strong>Command Basics</strong></td>
      <td>How commands and flags work</td>
      <td><a href="./commands/basics/README.md">Commands Basics</a></td>
    </tr>
    <tr>
      <td><strong>Auth Commands</strong></td>
      <td>Authenticate with the Flavortown API</td>
      <td><a href="./commands/auth/README.md">Auth Commands</a></td>
    </tr>
    <tr>
      <td><strong>Project Commands</strong></td>
      <td>Create, update, and manage projects and devlogs</td>
      <td><a href="./commands/project/README.md">Project Commands</a></td>
    </tr>
    <tr>
      <td><strong>Store Commands</strong></td>
      <td>Browse and query the store</td>
      <td><a href="./commands/store/README.md">Store Commands</a></td>
    </tr>
    <tr>
      <td><strong>User Commands</strong></td>
      <td>Fetch and list user information</td>
      <td><a href="./commands/user/README.md">User Commands</a></td>
    </tr>
  </tbody>
</table>

## Definitions
For the purpose of these docs, the following definitions will be used:
- **Commands** - Any action noun keyword that is used in the CLI to do things. I.E ``flavor user get``, where ``get`` is the command. For disambiguation, ``user get`` may be referred to as the command.
- **Subcommands** - Any noun keyword that is used to split up commands. I.E ``flavor user get``, where ``user`` is the subcommand.
- **Flags** - Any option for a command that is prefixed with a single `-` (short flags) or a double ``--`` (long flags). I.E ``flavor user get --help`` where ``--help`` is the flag. This includes flags that accept values, like ``flavor project create --title "cool project"``.
- **Options** - Any option for a command that *is not* prefixed with a `-` or `--`. I.E ``flavor user get 190``, where ``190`` is the option.