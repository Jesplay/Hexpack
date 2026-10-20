---
aliases: []
tags: [hexbook/entry, mod/hex-machina, category/patterns, topic/creatures, topic/devices]
mod: "Hex Machina"
category: "Patterns"
entry_id: "hexcasting:patterns/entity_capsule"
---

# Entity Capsule

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Machina]]

## [[Encapsulate|Encapsulate]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 60.8,15.8 38.3,15.8 15.8,15.8 27.0,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 60.8,15.8 38.3,15.8 15.8,15.8 27.0,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="35.3" x2="42.3" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.1,35.3 42.3,32.8 42.3,37.7" fill="#fc77be"/><circle cx="49.5" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`entity` → `—`

Reads a entity from the [[The Stack|stack]] and stores that entity in a entity capsule, costs 1 [[Amethyst|Amethyst Dust.]] per health point of the target entity

## [[Deploy|Deploy]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 38.3,15.8 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 38.3,15.8 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="35.3" x2="42.3" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.1,35.3 42.3,32.8 42.3,37.7" fill="#fc77be"/><circle cx="49.5" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`vec` → `—`

Reads a position from the stack and releases the entity from the entity capsule at that position, costs 1 [[Amethyst|Amethyst Shard.]]

## [[Ketchum's Reflection|Ketchum's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 38.3,15.8 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 38.3,54.7 27.0,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:3.9s" points="49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 38.3,15.8 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 38.3,54.7 27.0,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="35.3" x2="42.3" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.1,35.3 42.3,32.8 42.3,37.7" fill="#fc77be"/><circle cx="49.5" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`—` → `bool`

Reads if the entity capsule in the other hand has a entity stored

---
### Patterns on this page

- [[Encapsulate]] · Spell · Spell
- [[Deploy]] · Pattern · Other
- [[Ketchum's Reflection]] · Pattern · Reflection

**Sources:** 🧩 [Hex Machina on Modrinth](https://modrinth.com/mod/uQ6K0yH1)
