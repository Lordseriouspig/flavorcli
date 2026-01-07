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

# This script is primarily for use by Lordseriouspig, but it is provided here for transparency.
# This script will build the binaries for flavorcli on both windwows and linux (using wsl) and zip and release them to GitHub.
# It is meant to be used in conjunction with cargo release.
# This script requires the following dependencies to be installed:
# - cargo
# - wsl
# - GitHub CLI

$ErrorActionPreference = "Stop"

# Build binary for windows

# Get package info
$version = (cargo pkgid | ForEach-Object { ($_ -split "#")[-1] })
$packageName = "flavor"
$arch = "x86_64"
$platform = "windows"
$packageDir = "${packageName}cli-$version-$platform-$arch"

Write-Host "Building $packageName for Windows..."
cargo build --release

# Create temporary folder
if (Test-Path $packageDir) { Remove-Item -Recurse -Force $packageDir }
New-Item -ItemType Directory -Force -Path $packageDir

# Copy the binary
Copy-Item "target\release\$packageName.exe" "$packageDir\$packageName.exe" -Force

# Copy scripts
Copy-Item "install-win.ps1" "$packageDir\install-win.ps1" -Force
Copy-Item "uninstall-win.ps1" "$packageDir\uninstall-win.ps1" -Force

# Copy README, LICENSE and CHANGELOG
Copy-Item "README.md" "$packageDir" -Force
Copy-Item "LICENSE" "$packageDir" -Force
Copy-Item "CHANGELOG.md" "$packageDir" -Force

# Create the zip
$zipPath = "target\$packageDir.zip"
if (Test-Path $zipPath) { Remove-Item $zipPath -Force }

Add-Type -AssemblyName 'System.IO.Compression.FileSystem'
[System.IO.Compression.ZipFile]::CreateFromDirectory($packageDir, $zipPath)

# Clean up
Remove-Item -Recurse -Force $packageDir

Write-Host "Windows release zip created: $zipPath"

# And we do it again in wsl for linux

# Get package info
$arch = "x86_64"
$platform = "linux"
$packageDir = "${packageName}cli-$version-$platform-$arch"

Write-Host "Building $packageName for Linux..."
wsl /home/lachlan/.cargo/bin/cargo build --release

# Create temporary package folder
if (Test-Path $packageDir) { Remove-Item -Recurse -Force $packageDir }
New-Item -ItemType Directory -Force -Path $packageDir

# Copy the binary
Copy-Item "target\release\$packageName" "$packageDir\$packageName" -Force

# Copy install/uninstall scripts
Copy-Item "install-linux.sh" "$packageDir\install-linux.sh" -Force
Copy-Item "uninstall-linux.sh" "$packageDir\uninstall-linux.sh" -Force

# Copy README, LICENSE and CHANGELOG
Copy-Item "README.md" "$packageDir" -Force
Copy-Item "LICENSE" "$packageDir" -Force
Copy-Item "CHANGELOG.md" "$packageDir" -Force

# Create the zip
$zipPath = "target\$packageDir.zip"
if (Test-Path $zipPath) { Remove-Item $zipPath -Force }

Add-Type -AssemblyName 'System.IO.Compression.FileSystem'
[System.IO.Compression.ZipFile]::CreateFromDirectory($packageDir, $zipPath)

# Clean up temp folder
Remove-Item -Recurse -Force $packageDir

Write-Host "Linux release zip created: $zipPath"

# Release it to GitHub
git cliff --current | gh release create v$version "./target/${packageName}cli-$version-windows-$arch.zip" "./target/${packageName}cli-$version-linux-$arch.zip" -t "v$version" -F - --draft --prerelease=$($version -like "*-beta*" -or $version -like "*-alpha*" -or $version -like "*-rc*")
Write-Host "Release v$version created on GitHub. Check it out and publish it here: https://github.com/Lordseriouspig/flavorcli/releases/tag/v$version"