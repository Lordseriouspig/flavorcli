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

Commands are how you navigate through and perform actions using FlavorCLI. To understand how to use FlavorCLI, you first need to understand how commands work.

## Basic info
All commands and subcommands are noun-based (uses the imperative mood), and all have a ``--help`` flag so you can easily get quick info on the command. They also all include a ``--verbose`` (``-v``) and ``--quiet`` (``-q``) flag to increase or decrease logging verbosity based on the amount of times it was specified.

## Parts of a command
Commands are made up of a few parts:
<!-- yes i wrote this by hand-->
```bash
flavor project update 333 --title "cool project" --description "yey" --put
|      |       |      |   |                      |                   |
|      |       |      |   |                      |                   |
|      |       |      |   |                      |                   Boolean flags
|      |       |      |   |                      Additional Flags
|      |       |      |   Flags w/ values
|      |       |      Option
|      |       Command
|      Subcommand
Program
```

### Program
The program is the main program that you are running, which will always be ``flavor``. This can also be the path to the program if you did not add it to PATH.

### Subcommand
The subcommand is the "group" of commands that the command belongs to. If you want to manage projects, you would use project. If you want to view store items, you would use store.

### Options
Options are used in FlavorCLI to specify the target of the command. In the example, ``333`` is the project ID that we are going to edit.

### Flags w/ values
Flags with values are used when you have to supply something to the command. In the example, we supply the updated title and description to the command.

### Flags w/o values (boolean flags)
Flags without values will be true if provided and false if not. They are used for specifying settings, like in the example, where we specify ``--put`` to override the project instead of merging the changes with the old project. Some boolean flags can be specified multiple times to change the intensity of the flag. For example, in ``user get``, you can specify the ``--resolve`` (``-r``) once to resolve just their projects, or twice (like ``--resolve --resolve`` or ``-rr``) to resolve their projects and all of the devlogs on those projects.