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

If you want to be able to use FlavorCLI anywhere by typing ``flavor``, you must add FlavorCLI to PATH. There are two ways to do this.

## Using the install script.
The install script will move FlavorCLI to a new directory and add it to PATH. This is the easiest way to do so, and all you need to do is download the script and execute it in the same directory that you have the program in. The ZIP file you download comes with the script along with it.

## Manually
If you don't want to use the install script, simple follow the following steps:
1. Move the binary to a folder somewhere. We use ``~/bin`` on linux and ``%USERPROFILE%\.local\bin`` on windows.
### On Linux
2. Run ``export PATH=$PATH:/path/to/the/directory/``
## On Windows
2. Type "Edit the Environment Variables for Your Account" in search
3. Find the variable named "Path"
4. Click "Edit"
5. Click "New"
6. Click "Browse"
7. Add the directory
8. OK out of everything