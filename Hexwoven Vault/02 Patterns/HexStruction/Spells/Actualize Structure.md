---
aliases: ["Actualize Structure"]
tags: [hexcasting/spell, mod/hexstruction, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "HexStruction"
op_id: "hexstruction:load_structure"
signature: "aeqeawaeqea"
start_direction: "EAST"
input: "vec, structure"
output: ""
cost: "Costs about an eighth of one Amethyst Dust for every block placed."
iotas_in: ["[[Vector]]", "[[Structure]]"]
iotas_out: []
entry: "[[Structures]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Actualize Structure

✦ **Spell** · [[HexStruction]] · 📖 [[Structures]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,145.5 94.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 94.4,24.5 59.4,24.5 24.5,24.5 41.9,54.7 24.5,85.0 41.9,115.3 24.5,145.5 59.4,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:419;--dur:3.4s" points="59.4,145.5 94.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 94.4,24.5 59.4,24.5 24.5,24.5 41.9,54.7 24.5,85.0 41.9,115.3 24.5,145.5 59.4,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="145.5" x2="70.6" y2="145.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="78.6,145.5 70.6,149.4 70.6,141.7" fill="#fc77be"/><circle cx="59.4" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeqeawaeqea` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, structure` ([[Vector]], [[Structure]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about an eighth of one Amethyst Dust for every block placed.
- **ID:** `hexstruction:load_structure`

```text
before   … │ vec │ structure  ◂ top
after    …                    ◂ top
```

## Description

Removes the location of the structure's origin and a structure from the [[The Stack|stack]], then attempts to place the structure at the origin, erasing the structure from all memory. [[Mishaps|Mishaps]] if any part of the structure overlaps with another block.
Costs about an eighth of one [[Amethyst|Amethyst Dust]] for every block placed.

## Related patterns (same book page)

- [[Devour Structure]] `vec, vec → structure`
- [[Visualize Structure]] `vec, structure, num → —`

## Sources

- 📖 **Guidebook:** [[Structures]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexStruction on Modrinth](https://modrinth.com/mod/dGRSMTTM)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpLoadStructure` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexstruction:load_structure`
