---
aliases: ["Dictionary Manipulation"]
tags: [hexbook/entry, mod/hexthings, category/patterns, topic/data-structures]
mod: "HexThings"
category: "Patterns"
entry_id: "hexcasting:patterns/dict"
---

# Dictionary Manipulation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[HexThings]]

## [[Impasse Reflection|Impasse Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="48.8,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="69.9" x2="44.3" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.9,56.2 47.0,60.3 41.5,63.5" fill="#fc77be"/><circle cx="48.8" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`—` → `dict`

Creates an empty Dictionary [[Iotas|iota]].

## [[Accessor's Purification|Accessor's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="75.7,14.3 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="75.7,14.3 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="75.7" y1="14.3" x2="72.4" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="70.1,24.1 70.5,18.9 74.4,21.1" fill="#fc77be"/><circle cx="75.7" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`dict` → `list`

Pushes a list of the keys present in the Dictionary.

## [[Accessed's Purification|Accessed's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="75.7" y1="49.7" x2="72.4" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="70.1,40.0 74.4,43.0 70.5,45.2" fill="#fc77be"/><circle cx="75.7" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`dict` → `list`

Pushes a list of the values present in the Dictionary.

## [[Entry Purification|Entry Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="75.7,14.3 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="75.7,14.3 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="75.7" y1="14.3" x2="72.4" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="70.1,24.1 70.5,18.9 74.4,21.1" fill="#fc77be"/><circle cx="75.7" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`dict` → `list`

Pushes a list of the key-value pairs in this Dictionary, each being a 2 element list.

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`dict` → `num`

Pushes the number of entries in the Dictionary

## [[Locator's Distillation|Locator's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:201;--dur:2.0s" points="34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="45.0" x2="43.7" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="50.3,45.0 43.7,48.2 43.7,41.8" fill="#fc77be"/><circle cx="34.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`dict, key` → `any`

Pushes the value present at the provided key, or [[Nullary Reflection|Null]] if no value was present.

## [[Excisor's Distillation|Excisor's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,47.1 44.9,41.3 49.3,43.8" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`dict, key` → `dict, any`

Removes the key-value pair from the Dictionary, pushing the previous value, or [[Nullary Reflection|Null]] if no value was present.

## [[Surgeon's Exaltation|Surgeon's Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="71.5,64.4 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 18.5,64.4 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="71.5,64.4 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 18.5,64.4 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.5" y1="64.4" x2="67.2" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="64.2,51.8 69.8,55.6 64.7,58.5" fill="#fc77be"/><circle cx="71.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`dict, key, any` → `dict`

Places the iota in the Dictionary at the specified key. Not every iota type is allowed to be used as a key, with notable the exceptions being Garbage and [[Iris' Gambit|Jump]] iota. Attempting to set the value of a key to [[Nullary Reflection|Null]] will remove the key the same way using [[Excisor's Distillation]] would.

---
### Patterns on this page

- [[Impasse Reflection]] · Pattern · Reflection
- [[Accessor's Purification]] · Pattern · Purification
- [[Accessed's Purification]] · Pattern · Purification
- [[Entry Purification]] · Pattern · Purification
- [[Length Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Locator's Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Excisor's Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Surgeon's Exaltation]] · Pattern · Exaltation *(from Hex Casting)*

**Sources:** 🧩 [HexThings on Modrinth](https://modrinth.com/mod/uRBPvIjK)
