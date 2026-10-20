---
aliases: ["Identifiers"]
tags: [hexbook/entry, mod/hexpose, category/scrying, topic/data-structures]
mod: "Hexpose"
category: "Scrying"
entry_id: "hexcasting:patterns/scrying/identifiers"
---

# Identifiers

> **Category:** Patterns › [[Scrying (Category)|Scrying]]  
> **Source mod:** [[Hexpose]]

The identifier [[Iotas|iota]] is Nature's solution to allowing **Hexes** to specifically identify types. Every type of entity, block, status effect, enchantment, sound, and more has a unique identifier attached which a **Hex** can retrieve and compare.

## [[Detective's Purification|Detective's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:158;--dur:2.0s" points="49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="54.7" x2="53.1" y2="48.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.7,44.0 55.3,49.7 51.0,47.3" fill="#fc77be"/><circle cx="49.5" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`identifiable` → `identifier`

Pushes the identifier of an entity or block at a vector.

## [[Modicum Purification|Modicum Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,54.7 27.0,54.7 15.8,35.3 38.3,35.3 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:158;--dur:2.0s" points="49.5,54.7 27.0,54.7 15.8,35.3 38.3,35.3 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="54.7" x2="42.3" y2="54.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.1,54.7 42.3,52.3 42.3,57.2" fill="#fc77be"/><circle cx="49.5" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`any` → `identifier`

Pushes the identifier of an iota's type. Can be used to determine whether two iota are the same type.

---
### Patterns on this page

- [[Detective's Purification]] · Pattern · Purification
- [[Modicum Purification]] · Pattern · Purification

**Sources:** 🧩 [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
