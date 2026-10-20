---
aliases: []
tags: [hexbook/entry, mod/moreiotas, category/patterns, topic/data-structures]
mod: "MoreIotas"
category: "Patterns"
entry_id: "hexcasting:patterns/types@moreiotas"
---

# Types

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[MoreIotas]]

> [!note] Two versions of this page
> More than one mod ships a book page with this ID, so the in-game book shows only one of them (depending on load order). The other version: [[Types]].

## [[Classifier's Purification|Classifier's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,18.5 18.5,41.5 45.0,41.5 71.5,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:106;--dur:1.6s" points="31.8,18.5 18.5,41.5 45.0,41.5 71.5,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="18.5" x2="27.5" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="24.5,31.1 25.0,24.4 30.1,27.3" fill="#fc77be"/><circle cx="31.8" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`any` → `iotatype`

Remove an [[Iotas|iota]] from the [[The Stack|stack]], and adds the type of that iota to the stack.

## [[Physician's Purification|Physician's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,18.5 18.5,41.5 31.8,64.4 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="31.8,18.5 18.5,41.5 31.8,64.4 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="18.5" x2="27.5" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="24.5,31.1 25.0,24.4 30.1,27.3" fill="#fc77be"/><circle cx="31.8" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`entity` → `entitytype`

Remove an entity from the stack, and adds the type of the entity at that location to the stack.

## [[Sorter's Purification|Sorter's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,41.5 45.0,41.5 58.2,18.5 31.8,18.5 18.5,41.5 31.8,64.4 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="18.5,41.5 45.0,41.5 58.2,18.5 31.8,18.5 18.5,41.5 31.8,64.4 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="41.5" x2="27.0" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.1,41.5 27.0,44.4 27.0,38.5" fill="#fc77be"/><circle cx="18.5" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`itemtypable` → `itemtype`

Remove an iota with an associated item type (for example a location and the type of the block at that location, an item entity, or an item frame) from the stack and returns the item type to the stack (e.g. Grass, Stone, Stick, etc.).

## [[Sorter's Reflection|Sorter's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="58.2,18.5 45.0,41.5 18.5,41.5 31.8,18.5 58.2,18.5 71.5,41.5 45.0,41.5 31.8,64.4 18.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="58.2,18.5 45.0,41.5 18.5,41.5 31.8,18.5 58.2,18.5 71.5,41.5 45.0,41.5 31.8,64.4 18.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="58.2" y1="18.5" x2="54.0" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="51.0,31.1 51.5,24.4 56.5,27.3" fill="#fc77be"/><circle cx="58.2" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`—` → `itemtype`

Adds the type of the item in the caster's offhand to the stack if there is one present, or the type of the casting hand otherwise.

## [[Entity Distillation Type|Entity Distillation: Type]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="12.9,44.7 22.0,28.8 31.2,44.7 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 31.2,12.9 22.0,28.8 12.9,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:220;--dur:3.4s" points="12.9,44.7 22.0,28.8 31.2,44.7 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 31.2,12.9 22.0,28.8 12.9,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="44.7" x2="15.8" y2="39.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.9,35.9 17.5,40.6 14.0,38.6" fill="#fc77be"/><circle cx="12.9" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`entitytype, pos` → `entity | null`

Remove an entity type and a position from the stack, and returns an entity of that type at that position if one exists.

## [[Zone Exaltation Type|Zone Exaltation: Type]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="12.9,12.9 22.0,28.8 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 40.4,28.8 31.2,12.9 22.0,28.8 12.9,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:220;--dur:3.4s" points="12.9,12.9 22.0,28.8 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 40.4,28.8 31.2,12.9 22.0,28.8 12.9,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="12.9" x2="15.8" y2="17.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.9,21.6 14.0,19.0 17.5,16.9" fill="#fc77be"/><circle cx="12.9" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`entitytype, pos, num` → `[entity]`

Remove an entity type, a position, and a radius from the stack, and returns a list of all entities of that type within that radius of that position.

## [[Zone Exaltation Not-Type|Zone Exaltation: Not-Type]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="12.9,44.7 22.0,28.8 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 31.2,44.7 22.0,28.8 12.9,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:220;--dur:3.4s" points="12.9,44.7 22.0,28.8 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 31.2,44.7 22.0,28.8 12.9,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="44.7" x2="15.8" y2="39.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.9,35.9 17.5,40.6 14.0,38.6" fill="#fc77be"/><circle cx="12.9" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`entitytype, pos, num` → `[entity]`

Remove an entity type, a position, and a radius from the stack, and returns a list of all entities not of that type within that radius of that position

---
### Patterns on this page

- [[Classifier's Purification]] · Pattern · Purification
- [[Physician's Purification]] · Pattern · Purification
- [[Sorter's Purification]] · Pattern · Purification
- [[Sorter's Reflection]] · Pattern · Reflection
- [[Entity Distillation Type]] · Pattern · Distillation
- [[Zone Exaltation Type]] · Pattern · Exaltation
- [[Zone Exaltation Not-Type]] · Pattern · Exaltation

**Sources:** 🧩 [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
