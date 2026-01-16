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

git cliff -o "CHANGELOG.md";

(Get-Content ./PKGBUILD) | 
    ForEach-Object {
        $_ -replace '(pkgver=\d+\.\d+\.\d+)-', '${1}_'
    } | 
    Set-Content ./PKGBUILD