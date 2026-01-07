// Copyright (C) 2026 Lordseriouspig
// 
// This file is part of flavorcli.
// 
// flavorcli is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// flavorcli is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with flavorcli.  If not, see <https://www.gnu.org/licenses/>.

#[macro_export]
macro_rules! title {
    ($str:expr) => {
        println!(
            "{}\n{}",
            $str.bold().yellow(),
            "-".repeat(40)
        );
    };
}

#[macro_export]
macro_rules! heading {
    ($str:expr) => {
        println!("\n{}", $str.bold().cyan());
    };
}

#[macro_export]
macro_rules! field {
    ($name:expr, $value:expr) => {
        println!("{:<12}: {}", $name.blue(), $value);
    };
}

#[macro_export]
macro_rules! field_long {
    ($name:expr, $value:expr) => {
        println!("{:<20}: {}", $name.blue(), $value);
    };
}

#[macro_export]
macro_rules! long_text {
    ($name:expr, $value:expr) => {
        println!("\n{}", $name.bold().cyan());
        println!("{}", textwrap::fill($value, 72));
    };
}

#[macro_export]
macro_rules! list {
    ($items:expr) => {
        for item in $items {
            println!("- {}", item);
        }
    };
}