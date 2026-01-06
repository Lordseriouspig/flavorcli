# Copyright (C) 2026 Lordseriouspig
# 
# This file is part of flavorcli.
# 
# flavorcli is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# flavorcli is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with flavorcli.  If not, see <https://www.gnu.org/licenses/>.

# This script requires the compiled binary to be in the same directory as the script. It is primarily for use in the release zips.

#!/bin/sh
set -e

BINARY_NAME="flavor"
INSTALL_DIR="$HOME/bin"

if [ ! -f "./$BINARY_NAME" ]; then
    echo "Error: $BINARY_NAME binary not found in the current directory."
    exit 1
fi

mkdir -p "$INSTALL_DIR"
mv ./$BINARY_NAME "$INSTALL_DIR/$BINARY_NAME"

# Add to PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    echo "Added $INSTALL_DIR to PATH. Reload your shell or run: source ~/.bashrc"
fi

echo "$BINARY_NAME installed successfully! Please restart your terminal for changes to PATH to take effect."
