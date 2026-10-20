---
aliases: ["Phiangle"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/devices]
kind: "Spell"
family: "Spell"
topics: ["Casting Devices"]
mod: "Lapisworks"
op_id: "lapisworks:phiangle"
signature: "wadqaqdawewadqaqdaw"
start_direction: "NORTH_EAST"
input: "vec, vec"
output: ""
cost: "Costs a flat three Charged Amethyst, and a scalable one Amel per 32 blocks of distance, with a minimum of one."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Media Condensing Unit]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Phiangle

✦ **Spell** · [[Lapisworks]] · 📖 [[Media Condensing Unit]] · Lapisworks › Imbuement

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="69" height="170" viewBox="0 0 69.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="2"/><circle cx="34.7" cy="14.3" r="2"/><circle cx="55.1" cy="14.3" r="2"/><circle cx="4.1" cy="32.0" r="2"/><circle cx="24.5" cy="32.0" r="2"/><circle cx="44.9" cy="32.0" r="2"/><circle cx="65.3" cy="32.0" r="2"/><circle cx="14.3" cy="49.6" r="2"/><circle cx="34.7" cy="49.6" r="2"/><circle cx="55.1" cy="49.6" r="2"/><circle cx="4.1" cy="67.3" r="2"/><circle cx="24.5" cy="67.3" r="2"/><circle cx="44.9" cy="67.3" r="2"/><circle cx="65.3" cy="67.3" r="2"/><circle cx="14.3" cy="85.0" r="2"/><circle cx="34.7" cy="85.0" r="2"/><circle cx="55.1" cy="85.0" r="2"/><circle cx="4.1" cy="102.7" r="2"/><circle cx="24.5" cy="102.7" r="2"/><circle cx="44.9" cy="102.7" r="2"/><circle cx="65.3" cy="102.7" r="2"/><circle cx="14.3" cy="120.4" r="2"/><circle cx="34.7" cy="120.4" r="2"/><circle cx="55.1" cy="120.4" r="2"/><circle cx="4.1" cy="138.0" r="2"/><circle cx="24.5" cy="138.0" r="2"/><circle cx="44.9" cy="138.0" r="2"/><circle cx="65.3" cy="138.0" r="2"/><circle cx="14.3" cy="155.7" r="2"/><circle cx="34.7" cy="155.7" r="2"/><circle cx="55.1" cy="155.7" r="2"/></g><polyline class="hexpat-base" points="34.7,85.0 44.9,67.3 55.1,49.6 34.7,49.6 44.9,32.0 34.7,14.3 24.5,32.0 34.7,49.6 14.3,49.6 24.5,67.3 34.7,85.0 24.5,102.7 14.3,120.4 34.7,120.4 24.5,138.0 34.7,155.7 44.9,138.0 34.7,120.4 55.1,120.4 44.9,102.7 34.7,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:408;--dur:5.6s" points="34.7,85.0 44.9,67.3 55.1,49.6 34.7,49.6 44.9,32.0 34.7,14.3 24.5,32.0 34.7,49.6 14.3,49.6 24.5,67.3 34.7,85.0 24.5,102.7 14.3,120.4 34.7,120.4 24.5,138.0 34.7,155.7 44.9,138.0 34.7,120.4 55.1,120.4 44.9,102.7 34.7,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.7" y1="85.0" x2="38.0" y2="79.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="40.3,75.3 39.9,80.5 36.0,78.2" fill="#fc77be"/><circle cx="34.7" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wadqaqdawewadqaqdaw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a flat three Charged Amethyst, and a scalable one Amel per 32 blocks of distance, with a minimum of one.
- **ID:** `lapisworks:phiangle`

```text
before   … │ vec │ vec  ◂ top
after    …              ◂ top
```

## Description

**Phiangles** two **phiangleables**. Costs a flat three **Charged Amethyst**, and a scalable one **Amel** per 32 blocks of distance, with a minimum of one.

## Related patterns (same book page)

- [[Dephiangle]] `vec, vec → —`
- [[Phianglement Purification]] `vec → [vec]`

## Sources

- 📖 **Guidebook:** [[Media Condensing Unit]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `LinkCondensers` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:phiangle`
