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

> [!CAUTION]
> FlavorCLI *doesn't actually work* as of right now. I am awaiting for more features to be added to the Flavortown API before continuing the project. Until then, you **should not** use this project.

FlavorCLI is a fully-featured implementation of the Flavortown API, allowing for easier and streamlined interaction with HackClub's Flavortown.

## Installation
### Prebuilt (Recommended)
Upon release, FlavorCLI will be available on all[^1] major package managers.
[^1]: What that constitutes is unknown to me, *I'll figure it out when it's time™*

### Build it yourself
1. Download the source code or git clone the repo[^2]
2. Download and install rust if it is not already installed
3. Run ``cargo install --path .``
4. Profit!
[^2]: What you do is up to you, just get a copy of the code

## Usage
### Get your API key
1. To get your Flavortown API key, go to https://flavortown.hackclub.com/kitchen and do all the things to get yourself signed in.
2. Click the gear icon in the toolbar to open your settings
   
![An image with a bunch of arrows pointing to a gear icon that you can find in the sidebar on the flavortown website where you see your nameplate.](https://hc-cdn.hel1.your-objectstorage.com/s/v3/2578e86dd1cf0aa8_image.png)

3. Copy your API Key
   
![An image of the settings modal, the section you're looking for (The API key section btw) is at the bottom. That should be enough to help you find it, right?](https://hc-cdn.hel1.your-objectstorage.com/s/v3/f6e817ef5ec2b794_image.png)

4. Get your User ID. Either hover over the link, and note down the number that shows, or click on the link and note the number in the URL bar.
![An image pointing to your name in the sidebar.](https://hc-cdn.hel1.your-objectstorage.com/s/v3/69a83acd6aba05dc_image.png)

5. Run ``ft auth set <USER_ID> <TOKEN>``[^3]
[^3]: You can use ``ft auth delete`` at any time to delete your API Key. Your API Key is stored on device with your device's keychain. If you want to set auth for each command, add the ``--auth <APIKEY>`` flag

### Using the commands
Uhm, just use ``ft --help`` ig. (TODO: write proper guides on how to use the commands that definitely exist)

## Features
* Stores and verifies your API Key in your OS' keychain when using ``ft auth set <USER_ID> <TOKEN>``.
* thats it rn lol.

### Planned Features
- [x] Actually *have* commands
- [ ] Make those commands reflect what can be done with the API, add **more**
- [x] Authorization with Flavortown.
- [x] API Key storage with keychain.
- [x] Actually handle the commands lol.
- [ ] API Implementation
- [ ] Pretty much everything else