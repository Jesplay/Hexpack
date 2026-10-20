---
aliases: ["Structure Iota"]
tags: [hexbook/entry, mod/hexstruction, category/patterns, topic/blocks, topic/data-structures]
mod: "HexStruction"
category: "Patterns"
entry_id: "hexcasting:patterns/structures"
---

# Structure Iota

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[HexStruction]]

Inspired by my ability to both break and place blocks, I've devised a method to expand those capabilities. By breaking down a portion of the world itself into the mere concept of that space, I can store entire regions in my mind as a Structure [[Iotas|iota]], which I can manipulate and place back into the world. Unfortunately, it appears nature rejects my attempts to store [[Geodes|Budding Amethyst]] and its ilk.

Nature prefers to conserve matter; as a result, every copy of the same Structure iota will reference the same structure. Once one is used to place the structure back into the world, no other copy will be able to do the same. Coercing the iota to a boolean will return whether or not the iota remains valid. When "devouring" a structure from the world, it appears to define an "origin" at the lower North-West corner of the supplied bounds. This appears to remain true even after applying any number of transformations to the structure.

## [[Mirror Purification Z|Mirror Purification Z]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 40.7,61.0 31.5,45.0 22.2,29.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:296;--dur:4.5s" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 40.7,61.0 31.5,45.0 22.2,29.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="37.4" y2="77.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.6,77.0 37.4,79.1 37.4,75.0" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`structure` → `structure`

Flips a stored Structure front-to-back, or across the Z-axis.

## [[Mirror Purification X|Mirror Purification X]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 22.2,61.0 31.5,45.0 40.7,29.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:296;--dur:4.5s" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 22.2,61.0 31.5,45.0 40.7,29.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="37.4" y2="77.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.6,77.0 37.4,79.1 37.4,75.0" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`structure` → `structure`

Flips a stored Structure left-to-right, or across the X-axis.

## [[Mirror Purification Y|Mirror Purification Y]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="36.7,55.0 53.3,55.0 61.7,69.4 70.0,55.0 78.3,40.5 61.7,40.5 53.3,26.1 36.7,26.1 28.3,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,55.0 45.0,40.5 53.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:233;--dur:3.9s" points="36.7,55.0 53.3,55.0 61.7,69.4 70.0,55.0 78.3,40.5 61.7,40.5 53.3,26.1 36.7,26.1 28.3,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,55.0 45.0,40.5 53.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="55.0" x2="42.0" y2="55.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="45.8,55.0 42.0,56.8 42.0,53.1" fill="#fc77be"/><circle cx="36.7" cy="55.0" r="2.6" fill="#fc77be"/></svg>

`structure` → `structure`

Flips a stored Structure top-to-bottom, or across the Y-axis.

## [[Rotation Purification Clockwise Y|Rotation Purification Clockwise Y]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 22.2,61.0 31.5,45.0 40.7,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:296;--dur:4.5s" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 22.2,61.0 31.5,45.0 40.7,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="37.4" y2="77.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.6,77.0 37.4,79.1 37.4,75.0" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`structure` → `structure`

Rotates a stored Structure 90-degrees clockwise about the Y-axis.

## [[Rotation Purification Counter-Clockwise Y|Rotation Purification Counter-Clockwise Y]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 40.7,61.0 31.5,45.0 22.2,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:296;--dur:4.5s" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 40.7,61.0 31.5,45.0 22.2,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="37.4" y2="77.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.6,77.0 37.4,79.1 37.4,75.0" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`structure` → `structure`

Rotates a stored Structure 90-degrees counterclockwise about the Y-axis.

## [[Rotation Purification Clockwise X|Rotation Purification Clockwise X]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="46" height="90" viewBox="0 0 46.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.6" cy="9.6" r="1.6"/><circle cx="23.2" cy="9.6" r="1.6"/><circle cx="36.8" cy="9.6" r="1.6"/><circle cx="2.7" cy="21.4" r="1.6"/><circle cx="16.4" cy="21.4" r="1.6"/><circle cx="30.0" cy="21.4" r="1.6"/><circle cx="43.7" cy="21.4" r="1.6"/><circle cx="9.6" cy="33.2" r="1.6"/><circle cx="23.2" cy="33.2" r="1.6"/><circle cx="36.8" cy="33.2" r="1.6"/><circle cx="2.7" cy="45.0" r="1.6"/><circle cx="16.4" cy="45.0" r="1.6"/><circle cx="30.0" cy="45.0" r="1.6"/><circle cx="43.7" cy="45.0" r="1.6"/><circle cx="9.6" cy="56.8" r="1.6"/><circle cx="23.2" cy="56.8" r="1.6"/><circle cx="36.8" cy="56.8" r="1.6"/><circle cx="2.7" cy="68.6" r="1.6"/><circle cx="16.4" cy="68.6" r="1.6"/><circle cx="30.0" cy="68.6" r="1.6"/><circle cx="43.7" cy="68.6" r="1.6"/><circle cx="9.6" cy="80.4" r="1.6"/><circle cx="23.2" cy="80.4" r="1.6"/><circle cx="36.8" cy="80.4" r="1.6"/></g><polyline class="hexpat-base" points="23.2,56.8 36.8,56.8 30.0,45.0 36.8,33.2 30.0,21.4 36.8,9.6 23.2,9.6 9.6,9.6 16.4,21.4 9.6,33.2 16.4,45.0 9.6,56.8 23.2,56.8 30.0,68.6 23.2,80.4 16.4,68.6 30.0,68.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:218;--dur:4.5s" points="23.2,56.8 36.8,56.8 30.0,45.0 36.8,33.2 30.0,21.4 36.8,9.6 23.2,9.6 9.6,9.6 16.4,21.4 9.6,33.2 16.4,45.0 9.6,56.8 23.2,56.8 30.0,68.6 23.2,80.4 16.4,68.6 30.0,68.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="23.2" y1="56.8" x2="27.6" y2="56.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.7,56.8 27.6,58.3 27.6,55.3" fill="#fc77be"/><circle cx="23.2" cy="56.8" r="2.6" fill="#fc77be"/></svg>

`structure` → `structure`

Rotates a stored Structure 90-degrees clockwise about the X-axis.

## [[Rotation Purification Counter-Clockwise X|Rotation Purification Counter-Clockwise X]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="46" height="90" viewBox="0 0 46.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.6" cy="9.6" r="1.6"/><circle cx="23.2" cy="9.6" r="1.6"/><circle cx="36.8" cy="9.6" r="1.6"/><circle cx="2.7" cy="21.4" r="1.6"/><circle cx="16.4" cy="21.4" r="1.6"/><circle cx="30.0" cy="21.4" r="1.6"/><circle cx="43.7" cy="21.4" r="1.6"/><circle cx="9.6" cy="33.2" r="1.6"/><circle cx="23.2" cy="33.2" r="1.6"/><circle cx="36.8" cy="33.2" r="1.6"/><circle cx="2.7" cy="45.0" r="1.6"/><circle cx="16.4" cy="45.0" r="1.6"/><circle cx="30.0" cy="45.0" r="1.6"/><circle cx="43.7" cy="45.0" r="1.6"/><circle cx="9.6" cy="56.8" r="1.6"/><circle cx="23.2" cy="56.8" r="1.6"/><circle cx="36.8" cy="56.8" r="1.6"/><circle cx="2.7" cy="68.6" r="1.6"/><circle cx="16.4" cy="68.6" r="1.6"/><circle cx="30.0" cy="68.6" r="1.6"/><circle cx="43.7" cy="68.6" r="1.6"/><circle cx="9.6" cy="80.4" r="1.6"/><circle cx="23.2" cy="80.4" r="1.6"/><circle cx="36.8" cy="80.4" r="1.6"/></g><polyline class="hexpat-base" points="23.2,56.8 36.8,56.8 30.0,45.0 36.8,33.2 30.0,21.4 36.8,9.6 23.2,9.6 9.6,9.6 16.4,21.4 9.6,33.2 16.4,45.0 9.6,56.8 23.2,56.8 16.4,68.6 23.2,80.4 30.0,68.6 16.4,68.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:218;--dur:4.5s" points="23.2,56.8 36.8,56.8 30.0,45.0 36.8,33.2 30.0,21.4 36.8,9.6 23.2,9.6 9.6,9.6 16.4,21.4 9.6,33.2 16.4,45.0 9.6,56.8 23.2,56.8 16.4,68.6 23.2,80.4 30.0,68.6 16.4,68.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="23.2" y1="56.8" x2="27.6" y2="56.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.7,56.8 27.6,58.3 27.6,55.3" fill="#fc77be"/><circle cx="23.2" cy="56.8" r="2.6" fill="#fc77be"/></svg>

`structure` → `structure`

Rotates a stored Structure 90-degrees counterclockwise about the X-axis.

## [[Rotation Purification Clockwise Z|Rotation Purification Clockwise Z]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="46" height="90" viewBox="0 0 46.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.6" cy="9.6" r="1.6"/><circle cx="23.2" cy="9.6" r="1.6"/><circle cx="36.8" cy="9.6" r="1.6"/><circle cx="2.7" cy="21.4" r="1.6"/><circle cx="16.4" cy="21.4" r="1.6"/><circle cx="30.0" cy="21.4" r="1.6"/><circle cx="43.7" cy="21.4" r="1.6"/><circle cx="9.6" cy="33.2" r="1.6"/><circle cx="23.2" cy="33.2" r="1.6"/><circle cx="36.8" cy="33.2" r="1.6"/><circle cx="2.7" cy="45.0" r="1.6"/><circle cx="16.4" cy="45.0" r="1.6"/><circle cx="30.0" cy="45.0" r="1.6"/><circle cx="43.7" cy="45.0" r="1.6"/><circle cx="9.6" cy="56.8" r="1.6"/><circle cx="23.2" cy="56.8" r="1.6"/><circle cx="36.8" cy="56.8" r="1.6"/><circle cx="2.7" cy="68.6" r="1.6"/><circle cx="16.4" cy="68.6" r="1.6"/><circle cx="30.0" cy="68.6" r="1.6"/><circle cx="43.7" cy="68.6" r="1.6"/><circle cx="9.6" cy="80.4" r="1.6"/><circle cx="23.2" cy="80.4" r="1.6"/><circle cx="36.8" cy="80.4" r="1.6"/></g><polyline class="hexpat-base" points="23.2,56.8 36.8,56.8 30.0,45.0 36.8,33.2 30.0,21.4 36.8,9.6 23.2,9.6 9.6,9.6 16.4,21.4 9.6,33.2 16.4,45.0 9.6,56.8 23.2,56.8 30.0,68.6 23.2,80.4 16.4,68.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:4.2s" points="23.2,56.8 36.8,56.8 30.0,45.0 36.8,33.2 30.0,21.4 36.8,9.6 23.2,9.6 9.6,9.6 16.4,21.4 9.6,33.2 16.4,45.0 9.6,56.8 23.2,56.8 30.0,68.6 23.2,80.4 16.4,68.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="23.2" y1="56.8" x2="27.6" y2="56.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.7,56.8 27.6,58.3 27.6,55.3" fill="#fc77be"/><circle cx="23.2" cy="56.8" r="2.6" fill="#fc77be"/></svg>

`structure` → `structure`

Rotates a stored Structure 90-degrees clockwise about the Z-axis.

## [[Rotation Purification Counter-Clockwise Z|Rotation Purification Counter-Clockwise Z]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="46" height="90" viewBox="0 0 46.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.6" cy="9.6" r="1.6"/><circle cx="23.2" cy="9.6" r="1.6"/><circle cx="36.8" cy="9.6" r="1.6"/><circle cx="2.7" cy="21.4" r="1.6"/><circle cx="16.4" cy="21.4" r="1.6"/><circle cx="30.0" cy="21.4" r="1.6"/><circle cx="43.7" cy="21.4" r="1.6"/><circle cx="9.6" cy="33.2" r="1.6"/><circle cx="23.2" cy="33.2" r="1.6"/><circle cx="36.8" cy="33.2" r="1.6"/><circle cx="2.7" cy="45.0" r="1.6"/><circle cx="16.4" cy="45.0" r="1.6"/><circle cx="30.0" cy="45.0" r="1.6"/><circle cx="43.7" cy="45.0" r="1.6"/><circle cx="9.6" cy="56.8" r="1.6"/><circle cx="23.2" cy="56.8" r="1.6"/><circle cx="36.8" cy="56.8" r="1.6"/><circle cx="2.7" cy="68.6" r="1.6"/><circle cx="16.4" cy="68.6" r="1.6"/><circle cx="30.0" cy="68.6" r="1.6"/><circle cx="43.7" cy="68.6" r="1.6"/><circle cx="9.6" cy="80.4" r="1.6"/><circle cx="23.2" cy="80.4" r="1.6"/><circle cx="36.8" cy="80.4" r="1.6"/></g><polyline class="hexpat-base" points="23.2,56.8 36.8,56.8 30.0,45.0 36.8,33.2 30.0,21.4 36.8,9.6 23.2,9.6 9.6,9.6 16.4,21.4 9.6,33.2 16.4,45.0 9.6,56.8 23.2,56.8 16.4,68.6 23.2,80.4 30.0,68.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:4.2s" points="23.2,56.8 36.8,56.8 30.0,45.0 36.8,33.2 30.0,21.4 36.8,9.6 23.2,9.6 9.6,9.6 16.4,21.4 9.6,33.2 16.4,45.0 9.6,56.8 23.2,56.8 16.4,68.6 23.2,80.4 30.0,68.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="23.2" y1="56.8" x2="27.6" y2="56.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.7,56.8 27.6,58.3 27.6,55.3" fill="#fc77be"/><circle cx="23.2" cy="56.8" r="2.6" fill="#fc77be"/></svg>

`structure` → `structure`

Rotates a stored Structure 90-degrees counterclockwise about the Z-axis.

## [[Bounding Purification|Bounding Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 40.7,61.0 31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 22.2,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:370;--dur:5.6s" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 40.7,61.0 31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 22.2,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="37.4" y2="77.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.6,77.0 37.4,79.1 37.4,75.0" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`structure` → `[number]`

Removes a Structure from the [[The Stack|stack]], returning its dimensions in the format \[x, y, z\].

## [[Structure Transformation Purification|Structure Transformation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 22.2,61.0 31.5,45.0 50.0,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:4.2s" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 22.2,61.0 31.5,45.0 50.0,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="37.4" y2="77.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.6,77.0 37.4,79.1 37.4,75.0" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`structure` → `[number]`

Removes a Structure from the stack, returning its transformations as a list in the format \[mirror, vertical mirror, rotationX, rotationY, rotationZ\].

The values use the following key:
MIRROR
None -&gt; 0; Front-Back -&gt; 1; Left-Right -&gt; 2

VERTICAL MIRROR
Not Mirrored -&gt; 0; Mirrored -&gt; 1

ROTATION
No Rotation -&gt; 0; 90 CW -&gt; 1; 180 CW -&gt; 2; 90 CCW -&gt; 3

---
### Patterns on this page

- [[Mirror Purification Z]] · Pattern · Purification
- [[Mirror Purification X]] · Pattern · Purification
- [[Mirror Purification Y]] · Pattern · Purification
- [[Rotation Purification Clockwise Y]] · Pattern · Purification
- [[Rotation Purification Counter-Clockwise Y]] · Pattern · Purification
- [[Rotation Purification Clockwise X]] · Pattern · Purification
- [[Rotation Purification Counter-Clockwise X]] · Pattern · Purification
- [[Rotation Purification Clockwise Z]] · Pattern · Purification
- [[Rotation Purification Counter-Clockwise Z]] · Pattern · Purification
- [[Bounding Purification]] · Pattern · Purification
- [[Structure Transformation Purification]] · Pattern · Purification

**Sources:** 🧩 [HexStruction on Modrinth](https://modrinth.com/mod/dGRSMTTM)
