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

$binaryName = "flavor.exe"
$installDir = "$env:USERPROFILE\.local\bin"

if (!(Test-Path .\$binaryName)) {
    Write-Error "Error: $binaryName not found in the current directory."
    exit 1
}

if (!(Test-Path $installDir)) { New-Item -ItemType Directory -Path $installDir }

Move-Item -Force .\$binaryName $installDir

# Add to PATH
$oldPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($oldPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$installDir;$oldPath", "User")
}

Write-Host "$binaryName installed successfully! Please restart your terminal for changes to PATH to take effect."