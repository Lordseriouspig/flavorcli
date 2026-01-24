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

Store list allows you to list every item in the Flavortown store.

## Usage
```bash
flavor store list [OPTIONS]
```

### Options
-    **``--json``:**                       Returns data as raw JSON
-    **``--region <REGION>``:**           Region column to show [possible values: australia, canada, europe, india, united-kingdom, united-states, world]
-    **``--fields <FIELDS>``:**            Fields to output in the table (advanced) [default: id,name,description,stock,regional,type,attached-to] [possible values: id, name, description, stock, type, attached-to, limited, buyable-by-self, show-in-carousel, accessory-tag, long-description, image-url, max-qty, one-per-person-ever, sale-percentage, regional]
-    **``--sort <SORT>``:**                Sort the table output [default: id] [possible values: id, name, stock, type, limited, buyable-by-self, show-in-carousel, max-qty, one-per-person-ever, sale-percentage, regional]
-    **``--sort-region <SORT_REGION>``:**  Choose the region to sort by if you selected "regional" as the sort order [possible values: australia, canada, europe, india, united-kingdom, united-states, world]
-  **``-v, --verbose...``:**                 Increase logging verbosity
-  **``-q, --quiet...``:**                   Decrease logging verbosity
-  **``--sort-order <SORT_ORDER>``:**    Choose the direction of sort order [default: asc] [possible values: asc, desc]
-  **``-h, --help``:**                       Print help

## Examples
### List all store items
```bash
flavor store list
```
**Returns** (concatenated)
```
╭────┬────────────────────────────────┬────────────────────────────────────────────────────┬───────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────────────────────┬─────────────╮
│ ID ┆ Name                           ┆ Description                                        ┆ Stock ┆ AU     ┆ CA     ┆ EU     ┆ IN     ┆ UK     ┆ US     ┆ XX     ┆ Type                   ┆ Attached to │
╞════╪════════════════════════════════╪════════════════════════════════════════════════════╪═══════╪════════╪════════╪════════╪════════╪════════╪════════╪════════╪════════════════════════╪═════════════╡
│ 1  ┆ Stickers!                      ┆ we'll actually send you these!                     ┆ ∞     ┆ 10     ┆ 10     ┆ 10     ┆ 10     ┆ 10     ┆ 10     ┆ 10     ┆ FreeStickers           ┆             │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 2  ┆ Raspberry Pi 5                 ┆ mm,,, great for hosting all your yummy apps!       ┆ ∞     ┆ 405    ┆ 405    ┆ 405    ┆ 405    ┆ 405    ┆ 430    ┆ 455    ┆ ThirdPartyPhysical     ┆             │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 67 ┆ Candy Cane                     ┆ perfectly minty fresh                              ┆ ∞     ┆ 25     ┆ 25     ┆ 25     ┆ 25     ┆ 25     ┆ 25     ┆ 25     ┆ ThirdPartyPhysical     ┆             │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 68 ┆ Custom Santa Hat               ┆ a magical red hack club hat                        ┆ ∞     ┆ 50     ┆ 50     ┆ 50     ┆ 50     ┆ 50     ┆ 50     ┆ 50     ┆ SpecialFulfillmentItem ┆             │
╰────┴────────────────────────────────┴────────────────────────────────────────────────────┴───────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────────────────────┴─────────────╯
```

### List store items for a specific region
```bash
flavor store list --region united-states
```
**Returns** (concatenated)
```
╭────┬────────────────────────────────┬────────────────────────────────────────────────────┬───────┬────────┬────────────────────────┬─────────────╮
│ ID ┆ Name                           ┆ Description                                        ┆ Stock ┆ US     ┆ Type                   ┆ Attached to │
╞════╪════════════════════════════════╪════════════════════════════════════════════════════╪═══════╪════════╪════════════════════════╪═════════════╡
│ 1  ┆ Stickers!                      ┆ we'll actually send you these!                     ┆ ∞     ┆ 10     ┆ FreeStickers           ┆             │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 2  ┆ Raspberry Pi 5                 ┆ mm,,, great for hosting all your yummy apps!       ┆ ∞     ┆ 430    ┆ ThirdPartyPhysical     ┆             │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 67 ┆ Candy Cane                     ┆ perfectly minty fresh                              ┆ ∞     ┆ 25     ┆ ThirdPartyPhysical     ┆             │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 68 ┆ Custom Santa Hat               ┆ a magical red hack club hat                        ┆ ∞     ┆ 50     ┆ SpecialFulfillmentItem ┆             │
╰────┴────────────────────────────────┴────────────────────────────────────────────────────┴───────┴────────┴────────────────────────┴─────────────╯
```

### List store items with custom columns
```bash
flavor store list --fields id,name,stock,regional,show-in-carousel --region united-states
```
**Returns** (concatenated)
```
╭────┬────────────────────────────────┬───────┬────────┬──────────────────╮
│ ID ┆ Name                           ┆ Stock ┆ US     ┆ Show In Carousel │
╞════╪════════════════════════════════╪═══════╪════════╪══════════════════╡
│ 1  ┆ Stickers!                      ┆ ∞     ┆ 10     ┆ No               │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 2  ┆ Raspberry Pi 5                 ┆ ∞     ┆ 430    ┆ Yes              │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 67 ┆ Candy Cane                     ┆ ∞     ┆ 25     ┆ No               │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 68 ┆ Custom Santa Hat               ┆ ∞     ┆ 50     ┆ No               │
╰────┴────────────────────────────────┴───────┴────────┴──────────────────╯
```

### List store items with custom sort
```bash
flavor store list --sort regional --sort-region united-states --sort-order desc --region united-states
```
**Returns** (concatenated)
```
╭────┬────────────────────────────────┬────────────────────────────────────────────────────┬───────┬────────┬────────────────────────┬─────────────╮
│ ID ┆ Name                           ┆ Description                                        ┆ Stock ┆ US     ┆ Type                   ┆ Attached to │
╞════╪════════════════════════════════╪════════════════════════════════════════════════════╪═══════╪════════╪════════════════════════╪═════════════╡
│ 9  ┆ Macbook Air                    ┆ M4 13" - a favourite on our team!                  ┆ ∞     ┆ 4995   ┆ ThirdPartyPhysical     ┆             │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 60 ┆ Canon R50 Camera               ┆ a great starter camera! you can shoot all the b... ┆ ∞     ┆ 3690   ┆ ThirdPartyPhysical     ┆             │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 37 ┆ Framework Laptop 12            ┆ repairable touch screen laptop! diy edition        ┆ ∞     ┆ 3052   ┆ ThirdPartyPhysical     ┆             │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 10 ┆ Mac Mini                       ┆ M4 chip included!                                  ┆ ∞     ┆ 3000   ┆ ThirdPartyPhysical     ┆             │
╰────┴────────────────────────────────┴────────────────────────────────────────────────────┴───────┴────────┴────────────────────────┴─────────────╯
```

### List store items in JSON
```bash
flavor store list --json
```
**Returns** (concatenated)
```json
[{"id":9,"name":"Macbook Air","description":"M4 13\" - a favourite on our team!","old_prices":[],"limited":false,"stock":null,"type":"ShopItem::ThirdPartyPhysical","show_in_carousel":true,"accessory_tag":null,"agh_contents":"","attached_shop_item_ids":[null],"buyable_by_self":true,"long_description":null,"max_qty":null,"one_per_person_ever":false,"sale_percentage":null,"image_url":"https://flavortown.hackclub.com/rails/active_storage/blobs/proxy/eyJfcmFpbHMiOnsiZGF0YSI6MjUwLCJwdXIiOiJibG9iX2lkIn19--60d2d36d5c02546f61220f0814b67655a58de042/mba%20Background%20Removed%202.png","enabled":{"enabled_au":true,"enabled_ca":true,"enabled_eu":true,"enabled_in":true,"enabled_uk":true,"enabled_us":true,"enabled_xx":true},"ticket_cost":{"base_cost":4995.0,"au":4995.0,"ca":4995.0,"eu":4995.0,"in":4995.0,"uk":4995.0,"us":4995.0,"xx":4995.0}},{"id":2,"name":"Raspberry Pi 5","description":"mm,,, great for hosting all your yummy apps!","old_prices":[],"limited":false,"stock":null,"type":"ShopItem::ThirdPartyPhysical","show_in_carousel":true,"accessory_tag":null,"agh_contents":"","attached_shop_item_ids":[null],"buyable_by_self":true,"long_description":null,"max_qty":null,"one_per_person_ever":false,"sale_percentage":null,"image_url":"https://flavortown.hackclub.com/rails/active_storage/blobs/proxy/eyJfcmFpbHMiOnsiZGF0YSI6MjEwLCJwdXIiOiJibG9iX2lkIn19--bb4ab60784a2590933a9f7f273851b7c63f2b731/raspberry-pi-5-raspberry-pi-40958498898115_1000x%20Background%20Removed.png","enabled":{"enabled_au":true,"enabled_ca":true,"enabled_eu":true,"enabled_in":true,"enabled_uk":true,"enabled_us":true,"enabled_xx":true},"ticket_cost":{"base_cost":405.0,"au":405.0,"ca":405.0,"eu":405.0,"in":405.0,"uk":405.0,"us":430.0,"xx":455.0}}]
```