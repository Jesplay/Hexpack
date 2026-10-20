---
aliases: ["Status Iotas"]
tags: [hexbook/entry, mod/ephemera, category/patterns, topic/data-structures, topic/effects]
mod: "Ephemera"
category: "Patterns"
entry_id: "hexcasting:patterns/spells/status_iotas"
---

# Status Iotas

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Ephemera]]

These patterns work with a type of [[Iotas|iota]] that represents lingering effects on a creature, which can allow me to asses the state of a creature more thoroughly than with only [[Nurse's Purification]].
While my research suggests that status iotas representing instantaneous effects are possible, they would be very difficult to obtain, and I cannot think of any use case for them at the moment.

## [[Apothecary's Purification|Apothecary's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="35.3" x2="23.4" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,46.0 21.3,40.3 25.6,42.7" fill="#fc77be"/><circle cx="27.0" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`entity` → `[status]`

Accepts a living entity, and returns a list of status iotas representing all effects on the target.

## [[Apothecary's Purification II|Apothecary's Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,35.3 60.8,54.7 49.5,74.2 27.0,74.2 15.8,54.7 27.0,35.3 49.5,35.3 60.8,15.8 38.3,15.8 15.8,15.8 27.0,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="49.5,35.3 60.8,54.7 49.5,74.2 27.0,74.2 15.8,54.7 27.0,35.3 49.5,35.3 60.8,15.8 38.3,15.8 15.8,15.8 27.0,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="35.3" x2="53.1" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.7,46.0 51.0,42.7 55.3,40.3" fill="#fc77be"/><circle cx="49.5" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`status` → `num`

Accepts a status effect type, and returns a number based on whether the effect is likely to be helpful, harmful, or neutral to a creature.

## [[Pharmacist's Distillation|Pharmacist's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 38.3,54.7 27.0,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 38.3,54.7 27.0,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="35.3" x2="23.4" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,46.0 21.3,40.3 25.6,42.7" fill="#fc77be"/><circle cx="27.0" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`entity, status` → `num`

Accepts an entity and a status effect type, and returns the effect on the target's duration in seconds.

## [[Pharmacist's Dstln. II|Pharmacist's Dstln. II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,35.3 60.8,54.7 49.5,74.2 27.0,74.2 15.8,54.7 27.0,35.3 49.5,35.3 60.8,15.8 38.3,15.8 15.8,15.8 27.0,35.3 38.3,54.7 49.5,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="49.5,35.3 60.8,54.7 49.5,74.2 27.0,74.2 15.8,54.7 27.0,35.3 49.5,35.3 60.8,15.8 38.3,15.8 15.8,15.8 27.0,35.3 38.3,54.7 49.5,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="35.3" x2="53.1" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.7,46.0 51.0,42.7 55.3,40.3" fill="#fc77be"/><circle cx="49.5" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`entity, status` → `num`

Accepts an entity and a status effect type, and returns the effect on the target's potency.

## [[Entity Distillation Status|Entity Distillation: Status]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="12.9,12.9 31.2,12.9 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 40.4,28.8 31.2,44.7 12.9,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.8s" points="12.9,12.9 31.2,12.9 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 40.4,28.8 31.2,44.7 12.9,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="12.9" x2="18.7" y2="12.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.0,12.9 18.7,14.9 18.7,10.8" fill="#fc77be"/><circle cx="12.9" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`status, vector` → `entity | NULL`

Accepts a status effect type and a vector, and returns an entity at that position which has that effect.

## [[Zone Exaltation Status|Zone Exaltation: Status]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.8s" points="12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="12.9" x2="18.7" y2="12.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.0,12.9 18.7,14.9 18.7,10.8" fill="#fc77be"/><circle cx="12.9" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`status, vec, num` → `[entity]`

Accepts a status effect type, a vector, and a number (*n*), and returns a list of entities within radius *n* of the vector which have that effect.

## [[Zone Exaltation Not-Status|Zone Exaltation: Not-Status]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 40.4,28.8 31.2,12.9 12.9,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.8s" points="12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 40.4,28.8 31.2,12.9 12.9,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="44.7" x2="18.7" y2="44.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.0,44.7 18.7,46.7 18.7,42.6" fill="#fc77be"/><circle cx="12.9" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`status, vec, num` → `[entity]`

Accepts a status effect type, a vector, and a number (*n*), and returns a list of entities within radius *n* of the vector which do not have that effect.

## [[Ablation Purification|Ablation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 40.7,29.0 31.5,45.0 22.2,29.0 13.0,45.0 22.2,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:241;--dur:3.6s" points="31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 40.7,29.0 31.5,45.0 22.2,29.0 13.0,45.0 22.2,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="34.4" y2="71.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,68.2 36.2,72.9 32.7,70.9" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`entity` → `num`

Accepts a living entity, and returns how much damage it can Absorb before its health is affected.

## [[Cleanse Status|Cleanse Status]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="65.5,32.0 55.2,49.7 34.8,49.7 24.5,32.0 34.8,14.3 55.2,14.3 65.5,32.0 45.0,32.0 55.2,49.7 75.7,49.7 65.5,67.5 45.0,67.5 24.5,67.5 14.3,49.7 34.8,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="65.5,32.0 55.2,49.7 34.8,49.7 24.5,32.0 34.8,14.3 55.2,14.3 65.5,32.0 45.0,32.0 55.2,49.7 75.7,49.7 65.5,67.5 45.0,67.5 24.5,67.5 14.3,49.7 34.8,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="32.0" x2="62.2" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.8,41.8 60.2,36.6 64.1,38.8" fill="#fc77be"/><circle cx="65.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`entity, status` → `—`

Accepts an entity and a status effect type, and removes that status effect from the entity. Cost is based on the effect's innate harmfulness, its potency, and its remaining duration.

Costs more if attempting to remove a positive effect from an entity other than myself.

All of the patterns which deal with a specific status effect on an entity have the side effect that if the entity does not currently have the specified effect, the pattern will fail and inflict nausea upon me.

---
### Patterns on this page

- [[Apothecary's Purification]] · Pattern · Purification
- [[Apothecary's Purification II]] · Pattern · Purification
- [[Pharmacist's Distillation]] · Pattern · Distillation
- [[Pharmacist's Dstln. II]] · Pattern · Distillation
- [[Entity Distillation Status]] · Pattern · Distillation
- [[Zone Exaltation Status]] · Pattern · Exaltation
- [[Zone Exaltation Not-Status]] · Pattern · Exaltation
- [[Ablation Purification]] · Pattern · Purification
- [[Cleanse Status]] · Spell · Spell

**Sources:** 🌐 [Official online docs](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/status_iotas) · 🧩 [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
