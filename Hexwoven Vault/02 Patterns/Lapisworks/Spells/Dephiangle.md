---
aliases: ["Dephiangle"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/devices]
kind: "Spell"
family: "Spell"
topics: ["Casting Devices"]
mod: "Lapisworks"
op_id: "lapisworks:dephiangle"
signature: "wwqaqwwdwawwedeww"
start_direction: "SOUTH_WEST"
input: "vec, vec"
output: ""
cost: "Costs three Charged Amethyst."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Media Condensing Unit]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Dephiangle

✦ **Spell** · [[Lapisworks]] · 📖 [[Media Condensing Unit]] · Lapisworks › Imbuement

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="69" height="170" viewBox="0 0 69.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="2"/><circle cx="34.7" cy="14.3" r="2"/><circle cx="55.1" cy="14.3" r="2"/><circle cx="4.1" cy="32.0" r="2"/><circle cx="24.5" cy="32.0" r="2"/><circle cx="44.9" cy="32.0" r="2"/><circle cx="65.3" cy="32.0" r="2"/><circle cx="14.3" cy="49.6" r="2"/><circle cx="34.7" cy="49.6" r="2"/><circle cx="55.1" cy="49.6" r="2"/><circle cx="4.1" cy="67.3" r="2"/><circle cx="24.5" cy="67.3" r="2"/><circle cx="44.9" cy="67.3" r="2"/><circle cx="65.3" cy="67.3" r="2"/><circle cx="14.3" cy="85.0" r="2"/><circle cx="34.7" cy="85.0" r="2"/><circle cx="55.1" cy="85.0" r="2"/><circle cx="4.1" cy="102.7" r="2"/><circle cx="24.5" cy="102.7" r="2"/><circle cx="44.9" cy="102.7" r="2"/><circle cx="65.3" cy="102.7" r="2"/><circle cx="14.3" cy="120.4" r="2"/><circle cx="34.7" cy="120.4" r="2"/><circle cx="55.1" cy="120.4" r="2"/><circle cx="4.1" cy="138.0" r="2"/><circle cx="24.5" cy="138.0" r="2"/><circle cx="44.9" cy="138.0" r="2"/><circle cx="65.3" cy="138.0" r="2"/><circle cx="14.3" cy="155.7" r="2"/><circle cx="34.7" cy="155.7" r="2"/><circle cx="55.1" cy="155.7" r="2"/></g><polyline class="hexpat-base" points="55.1,85.0 44.9,102.7 34.7,120.4 24.5,138.0 34.7,155.7 44.9,138.0 34.7,120.4 24.5,102.7 14.3,85.0 34.7,85.0 55.1,85.0 44.9,67.3 34.7,49.6 24.5,32.0 34.7,14.3 44.9,32.0 34.7,49.6 24.5,67.3 14.3,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:367;--dur:5.0s" points="55.1,85.0 44.9,102.7 34.7,120.4 24.5,138.0 34.7,155.7 44.9,138.0 34.7,120.4 24.5,102.7 14.3,85.0 34.7,85.0 55.1,85.0 44.9,67.3 34.7,49.6 24.5,32.0 34.7,14.3 44.9,32.0 34.7,49.6 24.5,67.3 14.3,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.1" y1="85.0" x2="51.8" y2="90.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.5,94.7 49.9,89.5 53.8,91.8" fill="#fc77be"/><circle cx="55.1" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwqaqwwdwawwedeww` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs three Charged Amethyst.
- **ID:** `lapisworks:dephiangle`

```text
before   … │ vec │ vec  ◂ top
after    …              ◂ top
```

## Description

**Dephiangles** two **phiangleables**, erasing the link between them. Costs three **Charged Amethyst**. I can cast this when I wish not to break the block, due to having other links for example.

To my delight, the **Phianglement** granted to me by **Them** functions with [[Impeti]] as well! However, they may only have one **Phianglement** at any time and - perhaps to limit me? - they can only store a maximum of 10,000 dust without manual input. They automatically withdraw from the network to fulfill their own costs. Regardless, this means my [[Spell Circles]] may finally have a centralized [[Media|media]] storage!

## Related patterns (same book page)

- [[Phiangle]] `vec, vec → —`
- [[Phianglement Purification]] `vec → [vec]`

## Sources

- 📖 **Guidebook:** [[Media Condensing Unit]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `UnlinkCondensers` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:dephiangle`
