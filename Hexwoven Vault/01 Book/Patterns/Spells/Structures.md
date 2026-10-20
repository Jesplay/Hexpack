---
aliases: ["Structures"]
tags: [hexbook/entry, mod/hexstruction, category/spells, topic/blocks]
mod: "HexStruction"
category: "Spells"
entry_id: "hexcasting:patterns/spells/structure_spells"
---

# Structures

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[HexStruction]]

## [[Devour Structure|Devour Structure]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 13.0,77.0 22.2,61.0 13.0,45.0 22.2,29.0 13.0,13.0 31.5,13.0 50.0,13.0 40.7,29.0 50.0,45.0 40.7,61.0 50.0,77.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:222;--dur:3.4s" points="31.5,77.0 13.0,77.0 22.2,61.0 13.0,45.0 22.2,29.0 13.0,13.0 31.5,13.0 50.0,13.0 40.7,29.0 50.0,45.0 40.7,61.0 50.0,77.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="25.5" y2="77.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.3,77.0 25.5,75.0 25.5,79.1" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`vec, vec` → `structure`

Removes the locations of two corners from the [[The Stack|stack]], then consumes a region of space defined by those two corners, storing the result in a Structure [[Iotas|iota]].
Costs about an eighth of one [[Amethyst|Amethyst Dust]] for every block consumed.

## [[Actualize Structure|Actualize Structure]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:222;--dur:3.4s" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="37.4" y2="77.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.6,77.0 37.4,79.1 37.4,75.0" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`vec, structure` → `—`

Removes the location of the structure's origin and a structure from the stack, then attempts to place the structure at the origin, erasing the structure from all memory. [[Mishaps|Mishaps]] if any part of the structure overlaps with another block.
Costs about an eighth of one [[Amethyst|Amethyst Dust]] for every block placed.

## [[Visualize Structure|Visualize Structure]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="72" height="90" viewBox="0 0 72.2 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="68.5" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="68.5" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/><circle cx="68.5" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 40.7,61.0 59.2,61.0 50.0,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:4.2s" points="31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 40.7,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 40.7,61.0 59.2,61.0 50.0,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="37.4" y2="77.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.6,77.0 37.4,79.1 37.4,75.0" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`vec, structure, num` → `—`

Removes the location of the structure's origin and a structure from the stack, followed by a duration between 0 and 72000.

Displays a phantom structure at that location for the specified duration (in ticks). Does not erase the structure from memory.
Costs about a hundredth of one [[Amethyst|Amethyst Dust]] for every block displayed.

---
### Patterns on this page

- [[Devour Structure]] · Spell · Spell
- [[Actualize Structure]] · Spell · Spell
- [[Visualize Structure]] · Spell · Spell

**Sources:** 🧩 [HexStruction on Modrinth](https://modrinth.com/mod/dGRSMTTM)
