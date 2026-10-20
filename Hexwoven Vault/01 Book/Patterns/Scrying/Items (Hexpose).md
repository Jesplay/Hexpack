---
aliases: []
tags: [hexbook/entry, mod/hexpose, category/scrying, topic/items, topic/sensing, topic/text]
mod: "Hexpose"
category: "Scrying"
entry_id: "hexcasting:patterns/scrying/item"
---

# Items

> **Category:** Patterns › [[Scrying (Category)|Scrying]]  
> **Source mod:** [[Hexpose]]

An item [[Iotas|iota]] holds nearly all information that distinguish a [[The Stack|stack]] of items from another and **Hexes** can use them to query specifics. There is one exception: an item iota loses any item iota contained within the item. The already intricate folds of **[[Media|media]]** that form item iota simply unravel when nested within even more layers of abstraction.

**Players** manipulate **media** idly just as a byproduct of being sentient. This aura of **media** is mostly inert but strikes back, often violently at attempts of in-depth scrying. Externally exposed items such as armor and held items can be spied on, but attempts to deeply analyze their inventory will fail.

Nature respects boundaries.

## [[Item Purification|Item Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="71.5,41.5 45.0,41.5 31.8,18.5 58.2,18.5 71.5,41.5 58.2,64.4 45.0,41.5 18.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="71.5,41.5 45.0,41.5 31.8,18.5 58.2,18.5 71.5,41.5 58.2,64.4 45.0,41.5 18.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.5" y1="41.5" x2="63.0" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="56.9,41.5 63.0,38.5 63.0,44.4" fill="#fc77be"/><circle cx="71.5" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`entity` → `item`

Pushes the item that an **Item** in the world is.

## [[Offer Purification|Offer Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,41.5 45.0,41.5 58.2,18.5 31.8,18.5 18.5,41.5 31.8,64.4 45.0,41.5 71.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="18.5,41.5 45.0,41.5 58.2,18.5 31.8,18.5 18.5,41.5 31.8,64.4 45.0,41.5 71.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="41.5" x2="27.0" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.1,41.5 27.0,44.4 27.0,38.5" fill="#fc77be"/><circle cx="18.5" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`identifier, number` → `item`

Creates an item iota from an item identifier and the number of items in the stack.

## [[Tool Purification|Tool Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,64.4 45.0,41.5 31.8,18.5 18.5,41.5 31.8,64.4 58.2,64.4 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="31.8,64.4 45.0,41.5 31.8,18.5 18.5,41.5 31.8,64.4 58.2,64.4 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="64.4" x2="36.0" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.0,51.8 38.5,58.5 33.5,55.6" fill="#fc77be"/><circle cx="31.8" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`entity` → `item`

Returns the item in the entity's mainhand.

## [[Accessory Purification|Accessory Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="58.2,64.4 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 31.8,64.4 18.5,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="58.2,64.4 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 31.8,64.4 18.5,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="58.2" y1="64.4" x2="54.0" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="51.0,51.8 56.5,55.6 51.5,58.5" fill="#fc77be"/><circle cx="58.2" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`entity` → `item`

Returns the item in the entity's offhand.

## [[Cart Purification|Cart Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="60.8,54.7 38.3,54.7 27.0,35.3 49.5,35.3 60.8,54.7 49.5,74.2 27.0,74.2 15.8,54.7 27.0,35.3 15.8,15.8 38.3,15.8 49.5,35.3 38.3,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="60.8,54.7 38.3,54.7 27.0,35.3 49.5,35.3 60.8,54.7 49.5,74.2 27.0,74.2 15.8,54.7 27.0,35.3 15.8,15.8 38.3,15.8 49.5,35.3 38.3,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.8" y1="54.7" x2="53.6" y2="54.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="48.4,54.7 53.6,52.3 53.6,57.2" fill="#fc77be"/><circle cx="60.8" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`entity` → `list of items | null`

Returns a list of all items the entity is carrying, or Null if the entity can not hold items. Trying to access another **Player**'s inventory will incur a [[Mishaps|mishap]].

## [[Chest Purification|Chest Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,54.7 38.3,54.7 49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 60.8,15.8 38.3,15.8 27.0,35.3 38.3,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="15.8,54.7 38.3,54.7 49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 60.8,15.8 38.3,15.8 27.0,35.3 38.3,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="54.7" x2="23.0" y2="54.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.1,54.7 23.0,57.2 23.0,52.3" fill="#fc77be"/><circle cx="15.8" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`vector` → `list of items | null`

Returns a list of all items in the container, or Null if the block can not contain items.

## [[Aegis Purification|Aegis Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,74.2 49.5,54.7 38.3,35.3 27.0,54.7 38.3,74.2 15.8,74.2 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:293;--dur:3.6s" points="38.3,74.2 49.5,54.7 38.3,35.3 27.0,54.7 38.3,74.2 15.8,74.2 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="74.2" x2="41.9" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,63.5 44.0,69.2 39.7,66.8" fill="#fc77be"/><circle cx="38.3" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`entity` → `list of items`

Returns a list of all items the entity is wearing, from feet up.

## [[Pocket Reflection|Pocket Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="72" height="90" viewBox="0 0 72.2 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="68.5" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="68.5" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/><circle cx="68.5" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="50.0,45.0 59.2,29.0 50.0,13.0 40.7,29.0 50.0,45.0 31.5,45.0 22.2,61.0 40.7,61.0 50.0,45.0 59.2,61.0 50.0,77.0 31.5,77.0 22.2,61.0 13.0,45.0 22.2,29.0 31.5,45.0 40.7,61.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:296;--dur:4.5s" points="50.0,45.0 59.2,29.0 50.0,13.0 40.7,29.0 50.0,45.0 31.5,45.0 22.2,61.0 40.7,61.0 50.0,45.0 59.2,61.0 50.0,77.0 31.5,77.0 22.2,61.0 13.0,45.0 22.2,29.0 31.5,45.0 40.7,61.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.0" y1="45.0" x2="52.9" y2="39.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.0,36.2 54.7,40.9 51.2,38.9" fill="#fc77be"/><circle cx="50.0" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`—` → `list of items`

Returns a list of all items in my ender chest.

## [[Storage Purification|Storage Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,35.3 38.3,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2 60.8,74.2 72.0,54.7 60.8,35.3 49.5,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="15.8,35.3 38.3,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2 60.8,74.2 72.0,54.7 60.8,35.3 49.5,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="35.3" x2="23.0" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.1,35.3 23.0,37.7 23.0,32.8" fill="#fc77be"/><circle cx="15.8" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`item` → `number`

Takes in an item and pushes how many items are in it.

## [[Warehouse Purification|Warehouse Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="72.0,35.3 49.5,35.3 38.3,15.8 60.8,15.8 72.0,35.3 60.8,54.7 49.5,74.2 27.0,74.2 15.8,54.7 27.0,35.3 38.3,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="72.0,35.3 49.5,35.3 38.3,15.8 60.8,15.8 72.0,35.3 60.8,54.7 49.5,74.2 27.0,74.2 15.8,54.7 27.0,35.3 38.3,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.0" y1="35.3" x2="64.8" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.7,35.3 64.8,32.8 64.8,37.7" fill="#fc77be"/><circle cx="72.0" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`identifier` → `number`

Takes in an item and pushes how many items that can stack in it.

## [[Deterioration Purification|Deterioration Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 72.0,54.7 60.8,74.2 38.3,74.2 27.0,54.7 15.8,35.3 38.3,35.3 49.5,54.7 72.0,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:248;--dur:3.1s" points="15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 72.0,54.7 60.8,74.2 38.3,74.2 27.0,54.7 15.8,35.3 38.3,35.3 49.5,54.7 72.0,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="35.3" x2="19.4" y2="29.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,24.5 21.5,30.3 17.2,27.8" fill="#fc77be"/><circle cx="15.8" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`item` → `number`

Takes in an item and pushes how much damage the item has sustained.

## [[Fragility Purification|Fragility Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="72.0,35.3 60.8,15.8 38.3,15.8 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 72.0,35.3 49.5,35.3 38.3,54.7 15.8,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:248;--dur:3.1s" points="72.0,35.3 60.8,15.8 38.3,15.8 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 72.0,35.3 49.5,35.3 38.3,54.7 15.8,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.0" y1="35.3" x2="68.4" y2="29.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="65.8,24.5 70.6,27.8 66.3,30.3" fill="#fc77be"/><circle cx="72.0" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`identifier` → `number`

Takes in an item and pushes how much durability the item can have at its peak.

## [[Thaumaturgist's Purification|Thaumaturgist's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,11.7 28.3,11.7 11.7,11.7 20.0,26.1 36.7,26.1 45.0,40.5 53.3,26.1 70.0,26.1 78.3,11.7 61.7,11.7 45.0,11.7 36.7,26.1 28.3,40.5 45.0,40.5 61.7,40.5 53.3,26.1 45.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:267;--dur:4.5s" points="45.0,11.7 28.3,11.7 11.7,11.7 20.0,26.1 36.7,26.1 45.0,40.5 53.3,26.1 70.0,26.1 78.3,11.7 61.7,11.7 45.0,11.7 36.7,26.1 28.3,40.5 45.0,40.5 61.7,40.5 53.3,26.1 45.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="11.7" x2="39.7" y2="11.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.8,11.7 39.7,9.8 39.7,13.5" fill="#fc77be"/><circle cx="45.0" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`item` → `list of identifiers`

Gets a list of enchantments the item has.

## [[Charm Distillation|Charm Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="45.0,40.5 28.3,40.5 11.7,40.5 20.0,55.0 36.7,55.0 53.3,55.0 70.0,55.0 78.3,40.5 61.7,40.5 45.0,40.5 36.7,26.1 45.0,11.7 53.3,26.1 45.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="45.0,40.5 28.3,40.5 11.7,40.5 20.0,55.0 36.7,55.0 53.3,55.0 70.0,55.0 78.3,40.5 61.7,40.5 45.0,40.5 36.7,26.1 45.0,11.7 53.3,26.1 45.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="40.5" x2="39.7" y2="40.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.8,40.5 39.7,38.7 39.7,42.4" fill="#fc77be"/><circle cx="45.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`item, identifier` → `number`

Gets the strength of the specific enchantment on the item.

## [[Appellation Purification|Appellation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 45.0,32.0 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="24.5,32.0 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 45.0,32.0 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="27.8" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.2,41.8 25.9,38.8 29.8,36.6" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`item` → `text`

Gets the name given to this item.

## [[Legacy Purification|Legacy Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 24.5,32.0 45.0,32.0 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 24.5,32.0 45.0,32.0 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="21.3" y2="26.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,22.3 23.2,25.2 19.3,27.5" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`item` → `list of text`

Gets the lore written into this item.

## [[Glamour Purification|Glamour Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,74.2 15.8,74.2 27.0,54.7 38.3,35.3 15.8,35.3 27.0,54.7 38.3,74.2 49.5,54.7 38.3,35.3 27.0,15.8 15.8,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="38.3,74.2 15.8,74.2 27.0,54.7 38.3,35.3 15.8,35.3 27.0,54.7 38.3,74.2 49.5,54.7 38.3,35.3 27.0,15.8 15.8,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="74.2" x2="31.1" y2="74.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.9,74.2 31.1,71.8 31.1,76.7" fill="#fc77be"/><circle cx="38.3" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`item` → `number`

If this item is subceptible to [[Caster's Glamour (Book Entry)|Caster's Glamour]], pushes the index of the current form it's taking on. Otherwise pushes Null.

## [[Glamour Purification II|Glamour Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,74.2 27.0,74.2 38.3,54.7 49.5,35.3 27.0,35.3 38.3,54.7 49.5,74.2 60.8,54.7 49.5,35.3 38.3,15.8 27.0,35.3 15.8,54.7 27.0,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="49.5,74.2 27.0,74.2 38.3,54.7 49.5,35.3 27.0,35.3 38.3,54.7 49.5,74.2 60.8,54.7 49.5,35.3 38.3,15.8 27.0,35.3 15.8,54.7 27.0,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="74.2" x2="42.3" y2="74.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.1,74.2 42.3,71.8 42.3,76.7" fill="#fc77be"/><circle cx="49.5" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`item` → `number`

If this item is subceptible to [[Caster's Glamour (Book Entry)|Caster's Glamour]], pushes the number of forms it's capable of. Otherwise pushes Null.

## [[Reading Purification|Reading Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 45.0,32.0 34.8,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 45.0,32.0 34.8,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="28.2" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.5,14.3 28.2,12.1 28.2,16.6" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`item` → `list of text`

Pushes a list of [[Text Iota|text iotas]] contained inside a book or Null if it isn't a book.

## [[Bibliography Purification|Bibliography Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,14.3 34.8,14.3 45.0,32.0 55.2,14.3 75.7,14.3 65.5,32.0 55.2,49.7 34.8,49.7 24.5,32.0 14.3,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="14.3,14.3 34.8,14.3 45.0,32.0 55.2,14.3 75.7,14.3 65.5,32.0 55.2,49.7 34.8,49.7 24.5,32.0 14.3,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="14.3" x2="20.9" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,14.3 20.9,16.6 20.9,12.1" fill="#fc77be"/><circle cx="14.3" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`item` → `text, number`

Pushes the author of a book and its generation number.

## [[Collector Purification|Collector Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,13.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:111;--dur:1.7s" points="31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,13.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="34.4" y2="71.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,68.2 36.2,72.9 32.7,70.9" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`item` → `number`

Gauges the rarity of an item as a number, with greater values corresponding to a rarer item. Certain aspects such as enchantments can also elevate an item's rarity.

---
### Patterns on this page

- [[Item Purification]] · Pattern · Purification
- [[Offer Purification]] · Pattern · Purification
- [[Tool Purification]] · Pattern · Purification
- [[Accessory Purification]] · Pattern · Purification
- [[Cart Purification]] · Pattern · Purification
- [[Chest Purification]] · Pattern · Purification
- [[Aegis Purification]] · Pattern · Purification
- [[Pocket Reflection]] · Pattern · Reflection
- [[Storage Purification]] · Pattern · Purification
- [[Warehouse Purification]] · Pattern · Purification
- [[Deterioration Purification]] · Pattern · Purification
- [[Fragility Purification]] · Pattern · Purification
- [[Thaumaturgist's Purification]] · Pattern · Purification
- [[Charm Distillation]] · Pattern · Distillation
- [[Appellation Purification]] · Pattern · Purification
- [[Legacy Purification]] · Pattern · Purification
- [[Glamour Purification]] · Pattern · Purification
- [[Glamour Purification II]] · Pattern · Purification
- [[Reading Purification]] · Pattern · Purification
- [[Bibliography Purification]] · Pattern · Purification
- [[Collector Purification]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/item) · 🧩 [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
