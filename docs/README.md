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
{% hint style="info" %}
> [!TIP]
> This page is also available on GitBook! <!-- TODO: Insert link here -->
{% endhint %}

Welcome to the FlavorCLI docs! Here you may find all the information you need to use FlavorCLI.
To get started, take a look at the **Table of Contents** on the left. There you may find guides on many commands and concepts.

<!-- 
## Links
TODO: Insert common links here
>

<!--
## FAQ
TODO: Insert FAQ here...
-->

## Definitions
For the purpose of these docs, the following definitions will be used:
- **Commands** - Any action noun keyword that is used in the CLI to do things. I.E ``flavor user get``, where ``get`` is the command. For disambiguation, ``user get`` may be referred to as the command.
- **Subcommands** - Any noun keyword that is used to split up commands. I.E ``flavor user get``, where ``user`` is the subcommand.
- **Flags** - Any option for a command that is prefixed with a single `-` (short flags) or a double ``--`` (long flags). I.E ``flavor user get --help`` where ``--help`` is the flag. This includes flags that accept values, like ``flavor project create --title "cool project"``.
- **Options** - Any option for a command that *is not* prefixed with a `-` or `--`. I.E ``flavor user get 190``, where ``190`` is the option.