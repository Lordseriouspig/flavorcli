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

Thanks for choosing FlavorCLI!
There are many different ways to install FlavorCLI. Choose the one that is the easiest or most convenient to you.

## GitHub Releases
If you wish to download a pre-compiled version from GitHub releases, please follow the following steps:
1. Go to https://github.com/Lordseriouspig/flavorcli/releases/latest
2. Download the ZIP file corresponding to your OS.
3. Extract the contents to a folder, for example, a folder on your desktop.
4. Run the install script. (Ensure the binary file is in the same directory as the script)
5. Profit!
### Uninstall
1. Run the uninstall script. That's literally it.

## Package Managers
FlavorCLI is on a few package managers. These can be convenient if you already have them installed, and make for easy management of your packages.

### [Crates.io (Cargo)](https://crates.io/crates/flavorcli)
1. Ensure you have cargo installed. Follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you do not.
2. Run ``cargo install flavorcli``
3. Profit!

#### Uninstall
1. Ensure you *still* have cargo installed. Follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you uninstalled it.
2. Run ``cargo uninstall flavorcli``

## Compile from source (not recommended)
You have two options if you wish to install FlavorCLI from source.

### Download source from GitHub Releases
1. Go to https://github.com/Lordseriouspig/flavorcli/releases/latest
2. Download the source code file (either the .zip or .tar.gz)
3. Extract it somewhere
4. Ensure you have cargo installed. Follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you do not.
5. Run ``cargo build --release``

From here, you may choose to run the install script after moving the binary into the root directory, or move it somewhere and [add it to PATH manually](./add_to_path.md)

### Using git clone
1. Ensure you have git installed.
2. Run ``git clone https://github.com/Lordseriouspig/flavorcli``
3. Ensure you have cargo installed. Follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you do not.
4. Run ``cargo build --release``

From here, you may choose to run the install script after moving the binary into the root directory, or move it somewhere and [add it to PATH manually](./add_to_path.md)

## Downloading a development build
{% hint style="warning" %}
We **do not** recommend installing from the development branch. There is **no guarantee** that the build will be stable or work whatsoever. Things here change rapidly and may be half-complete or broken entirely.
{% endhint %}

If you want to try out the latest and greatest features, you may want to download from the development branch. Here's how to do that.
1. Ensure you have git installed.
2. Run ``git clone -b development https://github.com/Lordseriouspig/flavorcli``
3. Ensure you have cargo installed. Follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you do not.
4. Run ``cargo build --release``

From here, you may choose to run the install script after moving the binary into the root directory, or move it somewhere and [add it to PATH manually](./add_to_path.md).