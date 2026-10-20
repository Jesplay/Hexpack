---
aliases: ["Maps"]
tags: [hexbook/entry, mod/phlib, category/patterns, topic/data-structures]
mod: "PhLib"
category: "Patterns"
entry_id: "hexcasting:addon/phlib/maps"
---

# Maps

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[PhLib]]

Maps are an efficient structure holding pairings of some [[Iotas|iota]] to some other iota. While this is perfectly possible using merely list operations, dedicated map iotas use some arcane substance called a '**Hash**'. While this doesn't reduce the *operations* required to access a map, it desperately reduces the load on Nature, nearing O(1) for most access.

## [[Uncharted Reflection|Uncharted Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,18.5 71.5,18.5 58.2,41.5 71.5,64.4 45.0,64.4 18.5,64.4 31.8,41.5 18.5,18.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="45.0,18.5 71.5,18.5 58.2,41.5 71.5,64.4 45.0,64.4 18.5,64.4 31.8,41.5 18.5,18.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="18.5" x2="53.5" y2="18.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,18.5 53.5,21.4 53.5,15.6" fill="#fc77be"/><circle cx="45.0" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`—` → `map<⊥, ⊥>`

Creates a new empty map for my usage.

## [[Surgeon's Exaltation|Surgeon's Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="71.5,64.4 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 18.5,64.4 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="71.5,64.4 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 18.5,64.4 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.5" y1="64.4" x2="67.2" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="64.2,51.8 69.8,55.6 64.7,58.5" fill="#fc77be"/><circle cx="71.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`map<k, v>, k, v` → `map<k, v>`

Replaces the element associated with **k** with **v**. If **k** is not present in the map, the entry is created and placed at the end of the map.

## [[Excisor's Distillation|Excisor's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,47.1 44.9,41.3 49.3,43.8" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`map<k, v>, k` → `map<k, v>`

Throws out the map's association for the given key. If there is no matching association, the map is returned unchanged.

## [[Selection Distillation|Selection Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="45.0" x2="29.9" y2="37.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.6,31.3 32.6,35.5 27.1,38.6" fill="#fc77be"/><circle cx="34.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`map<k, v>, k` → `v?`

Destroys the map, returning only the value associated with the given key. O(1), unlike scanning a list.

## [[Flock's Disintegration|Flock's Disintegration]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,25.1 72.4,28.5 68.0,31.0" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`map<k, v>` → `k, v, k, v...`

Disintegrates the map, returning the key-value pairs in the order I inserted them.

## [[Derivation Decomposition|Derivation Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="63.2,45.0 48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9 63.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="63.2,45.0 48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9 63.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="63.2" y1="45.0" x2="58.6" y2="37.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.3,31.3 61.4,35.5 55.9,38.6" fill="#fc77be"/><circle cx="63.2" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`map<k,v>` → `map<k,v>, (k,v | nulls)`

Shaves off the element I added most recently from a map. If used on an empty map, returns two nulls.

## [[Speaker's Decomposition|Speaker's Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="65.5,14.3 55.2,32.0 75.7,32.0 65.5,14.3 45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 24.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="65.5,14.3 55.2,32.0 75.7,32.0 65.5,14.3 45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 24.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="14.3" x2="62.2" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.8,24.1 60.2,18.9 64.1,21.1" fill="#fc77be"/><circle cx="65.5" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`map<k,v>` → `map<k,v>, (k,v | nulls)`

Shaves off the element I added least recently from a map. If used on an empty map, returns two nulls.

## [[Additive Distillation|Additive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`map<k, v>, map<k, v>` → `map<k, v>`

Merges two maps together. Maps cannot have duplicate items: any entry in the latter map will override that in the first map.

## [[Subtractive Distillation|Subtractive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,69.9 34.5,45.0 20.1,20.1 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="48.8,69.9 34.5,45.0 20.1,20.1 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="69.9" x2="44.3" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.9,56.2 47.0,60.3 41.5,63.5" fill="#fc77be"/><circle cx="48.8" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`map<k, v>, map<t, *>` → `map<k, v>`

Removes every entry in the first map that is found in the second map. The values in the second map are ignored.

---
### Patterns on this page

- [[Uncharted Reflection]] · Pattern · Reflection
- [[Surgeon's Exaltation]] · Pattern · Exaltation *(from Hex Casting)*
- [[Excisor's Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Selection Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Flock's Disintegration]] · Pattern · Disintegration *(from Hex Casting)*
- [[Derivation Decomposition]] · Pattern · Decomposition *(from Hex Casting)*
- [[Speaker's Decomposition]] · Pattern · Decomposition *(from Hex Casting)*
- [[Additive Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Subtractive Distillation]] · Pattern · Distillation *(from Hex Casting)*
