---
aliases: ["Memory Crystal"]
tags: [hexbook/entry, mod/hexintent, category/items, topic/storage]
mod: "HexIntent"
category: "Items"
entry_id: "hexcasting:manifestation/memory_crystal"
---

# Memory Crystal

> **Category:** [[Items (Category)|Items]]  
> **Source mod:** [[HexIntent]]

### Memory Crystal

*Item:* **Memory Crystal**

Memory Crystals are funny little things. They have their own unique identifier and can store patterns within them.

I've found that you can jam these things into other magical items, keeping their properties. Although jamming two into one item often cracks the first.

I've found that you can jam these things into other magical items adding a form of persistent memory to them. Although jamming two into one item often cracks the first.

**Memory Crystal**: shaped crafting

|   |   |   |
|---|---|---|
|   | `A` |   |
| `A` | `F` | `A` |
|   | `A` |   |

`A` = Charged Amethyst · `F` = Focus

Forged from a [[Focus|focus]] and charged crystals. Makes sense given their properties.

## [[Resonance Reflection|Resonance Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,29.0 13.0,45.0 22.2,61.0 31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 40.7,61.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.8s" points="22.2,29.0 13.0,45.0 22.2,61.0 31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 40.7,61.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="29.0" x2="19.2" y2="34.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.1,37.8 17.5,33.1 21.0,35.1" fill="#fc77be"/><circle cx="22.2" cy="29.0" r="2.6" fill="#fc77be"/></svg>

`none` → `memory`

Returns the held or embedded Memory Crystal's unique memory identifier. Favours the off-hand first.

## [[Resonance Purification|Resonance Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,29.0 13.0,45.0 22.2,61.0 31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 31.5,13.0 22.2,29.0 40.7,29.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:167;--dur:2.5s" points="22.2,29.0 13.0,45.0 22.2,61.0 31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 31.5,13.0 22.2,29.0 40.7,29.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="29.0" x2="19.2" y2="34.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.1,37.8 17.5,33.1 21.0,35.1" fill="#fc77be"/><circle cx="22.2" cy="29.0" r="2.6" fill="#fc77be"/></svg>

`memory` → `[pattern] | null`

Returns the stored memory pattern (or null if none) from a held or embedded Memory Crystal. Takes a Memory [[Iotas|Iota]] from Memory Reflection. Favours the off-hand first.

## [[Resonance Gambit|Resonance Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,29.0 13.0,45.0 22.2,61.0 31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 40.7,29.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.8s" points="22.2,29.0 13.0,45.0 22.2,61.0 31.5,77.0 40.7,61.0 50.0,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 40.7,29.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="29.0" x2="19.2" y2="34.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.1,37.8 17.5,33.1 21.0,35.1" fill="#fc77be"/><circle cx="22.2" cy="29.0" r="2.6" fill="#fc77be"/></svg>

`memory, any` → `none`

Stores the top of the [[The Stack|stack]] in a held or embedded Memory Crystal's memory. Takes a Memory Iota from Memory Reflection. Favours the off-hand first.

---
### Patterns on this page

- [[Resonance Reflection]] · Pattern · Reflection
- [[Resonance Purification]] · Pattern · Purification
- [[Resonance Gambit]] · Pattern · Gambit

**Sources:** 🧩 [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
