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

The store get command allows you to get specific info about an item in the Flavortown Store.

## Usage
```
flavor store get [FLAGS] <ITEM_ID>
```

### Options
-  **``<ITEM_ID>``:**  The store item ID to retrieve

### Flags
-   **``   --json``:**        Returns data as raw JSON
-  **``-s, --short``:**       Omits some metadata
-  **``-d, --detailed``:**    Adds random other metadata
-  **``-v, --verbose...``:**  Increase logging verbosity
-  **``-q, --quiet...``:**    Decrease logging verbosity
-  **``-h, --help``:**        Print help

## Examples
### Get info about a specific item
```bash
flavor store get 67
```
**Returns**
```
Candy Cane
----------------------------------------
ID                  : 67
Stock               : ∞
Type                : ShopItem::ThirdPartyPhysical
Description         : perfectly minty fresh

Regional Info:
🇦🇺 Australia        : Enabled? true Price: 25
🇨🇦 Canada           : Enabled? true Price: 25
🇪🇺 Europe           : Enabled? true Price: 25
🇮🇳 India            : Enabled? true Price: 25
🇬🇧 United Kingdom   : Enabled? true Price: 25
🇺🇸 United States    : Enabled? true Price: 25
🇺🇳 Global           : Enabled? true Price: 25

Buying Info:
Limited?            : false
Stock               : ∞
One Per Person Ever?: true
Buyable By Self?    : true

Image URL
https://flavortown.hackclub.com/rails/active_storage/blobs/proxy/
eyJfcmFpbHMiOnsiZGF0YSI6MTMxMjIsInB1ciI6ImJsb2JfaWQifX0=--94135e727396fe
a94c4eaaf1599e115b6fc515e3/Candy-Cane-Classic-removebg-preview.png
```

### Get a specific item in JSON
```bash
flavor store get 67 --json
```
**Returns**
```json
{"id":67,"name":"Candy Cane","description":"perfectly minty fresh","old_prices":[],"limited":false,"stock":null,"type":"ShopItem::ThirdPartyPhysical","show_in_carousel":false,"accessory_tag":"","agh_contents":"","attached_shop_item_ids":[null],"buyable_by_self":true,"long_description":"","max_qty":null,"one_per_person_ever":true,"sale_percentage":null,"image_url":"https://flavortown.hackclub.com/rails/active_storage/blobs/proxy/eyJfcmFpbHMiOnsiZGF0YSI6MTMxMjIsInB1ciI6ImJsb2JfaWQifX0=--94135e727396fea94c4eaaf1599e115b6fc515e3/Candy-Cane-Classic-removebg-preview.png","enabled":{"enabled_au":true,"enabled_ca":true,"enabled_eu":true,"enabled_in":true,"enabled_uk":true,"enabled_us":true,"enabled_xx":true},"ticket_cost":{"base_cost":25.0,"au":25.0,"ca":25.0,"eu":25.0,"in":25.0,"uk":25.0,"us":25.0,"xx":25.0}}
```