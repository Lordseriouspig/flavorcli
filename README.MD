<!--
 Copyright (C) 2025 Lordseriouspig
 
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

# FlavorCLI
[![Made with Rust](https://img.shields.io/badge/Rust-%5E1.92.0-blue?logo=rust&logoColor=white)](https://www.rust-lang.org/ "Go to Rust homepage")
[![Licence](https://img.shields.io/github/license/lordseriouspig/flavorcli)](https://www.gnu.org/licenses/gpl-3.0.en.html "GNU GPL 3.0")
[![Tracked on Hackatime](https://hackatime-badge.hackclub.com/U096NGLTF4K/flavorcli)](https://hackatime.hackclub.com/ "Go to Hackatime homepage")
[![Commit activity](https://img.shields.io/github/commit-activity/w/lordseriouspig/flavorcli)](https://github.com/Lordseriouspig/flavorcli/commits/main/ "Commit History")
[![Last Commit](https://img.shields.io/github/last-commit/lordseriouspig/flavorcli)](https://github.com/Lordseriouspig/flavorcli/commits/main/ "Commit History")
[![rust-clippy analyze](https://github.com/Lordseriouspig/flavorcli/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Lordseriouspig/flavorcli/actions/workflows/rust-clippy.yml)

FlavorCLI is a fully-featured implementation of the Flavortown API, allowing for easier and streamlined interaction with HackClub's Flavortown.

# Full Documentation
The full documentation is available on [GitBook](https://flavorcli.gitbook.io/flavorcli-docs/) or in the [Docs Folder](/docs/). I've intentionally written these as more beginner friendly and less silly than this README.

# Quick Start
Please note that this quick start guide may not be beginner friendly and is a bit silly.

## Installation
### Prebuilt (Recommended)
At some point, FlavorCLI will be available on all[^1] major package managers.
#### GitHub Releases
To download and install precompiled binaries from GitHub, follow these steps:
1. Download the ZIP file for your platform
2. Unzip the contents
3. Inside, you will see the precompiled binary, plus this readme, a copy of the licence, and an installer and uninstaller script for convenience.
4. (Optional) Execute the installer script.
   1. If you are on windows, run ``powershell.exe -ExecutionPolicy Bypass -File "install-win.ps1"``
   2. If you are on linux, run ``chmod +x install-linux.sh``
   3. Actually run the script
   4. This will move the binary to a bin directory and add it to PATH, so you can type ``flavor`` to run the app at any time.
5. Keep the uninstall script handy for once flavortown ends.
[^1]: What that constitutes is unknown to me, *I'll figure it out when it's time™*

#### Cargo
If you have cargo, you may choose to install FlavorCLI with the following command:
```
cargo install flavorcli
```
Ensure you have the cargo install directory in your PATH.

#### AUR
If you use Arch Linux, you can build using AUR with this command:
```
yay -S flavorcli
```

### Build it yourself
1. Download the source code or git clone the repo[^2]
2. Download and install rust if it is not already installed
3. Run ``cargo build -r``
4. Run ``cargo install --path .``
5. Ensure you have your cargo's install directory (usually ``~/.cargo/bin``) in PATH
6. Profit!
[^2]: What you do is up to you, just get a copy of the code

## Usage
### Get your API key
1. To get your Flavortown API key, go to https://flavortown.hackclub.com/kitchen and do all the things to get yourself signed in.
2. Click the gear icon in the toolbar to open your settings
   
![An image with a bunch of arrows pointing to a gear icon that you can find in the sidebar on the flavortown website where you see your nameplate.](https://hc-cdn.hel1.your-objectstorage.com/s/v3/2578e86dd1cf0aa8_image.png)

3. Copy your API Key
   
![An image of the settings modal, the section you're looking for (The API key section btw) is at the bottom. That should be enough to help you find it, right?](https://hc-cdn.hel1.your-objectstorage.com/s/v3/f6e817ef5ec2b794_image.png)

4. FlavorCLI no longer requires you to get your user ID! Having said that, if you do wish to use the ``--no-verify`` tag when setting your auth, you *do* still need to add your user ID. You may also choose to override the user ID linked to your token for some reason.

1. Run ``ft auth set <TOKEN>``[^3]
[^3]: You can use ``ft auth delete`` at any time to delete your API Key. Your API Key is stored on device with your device's keychain. If you want to set auth for each command, add the ``--auth <APIKEY>`` flag

## Features
* Stores and verifies your API Key in your OS' keychain when using ``ft auth set <USER_ID> <TOKEN>``.
* Get and list projects
* Get and list devlogs
* Get and list store items
* Get and list users
* Manage Projects
* Create and Manage Devlogs!!!
* Customisable tables and sorting

### Planned Features
When (if) the routes are added, I'll be adding the following features:
[x] Manage Devlogs
[ ] Change your settings
[ ] View your votes

Once I finish that, I'll ship the project and work on:
[ ] Full TUI
[ ] Possibly more depending on feedback