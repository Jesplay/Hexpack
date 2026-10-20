---
aliases: ["Push"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:piston"
signature: "wqwawqwqqqeqq"
start_direction: "SOUTH_WEST"
input: "vector, vector"
output: ""
cost: "Costs an eighth of an Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Block Mimicry]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Push

✦ **Spell** · [[Hexical]] · 📖 [[Block Mimicry]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,24.5 59.4,54.7 41.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 111.8,85.0 94.4,54.7 76.9,24.5 41.9,24.5 24.5,54.7 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:489;--dur:3.9s" points="76.9,24.5 59.4,54.7 41.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 111.8,85.0 94.4,54.7 76.9,24.5 41.9,24.5 24.5,54.7 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="24.5" x2="71.3" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.3,41.1 68.0,32.2 74.6,36.1" fill="#fc77be"/><circle cx="76.9" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwawqwqqqeqq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs an eighth of an Amethyst Dust.
- **ID:** `hexical:piston`

```text
before   … │ vector │ vector  ◂ top
after    …                    ◂ top
```

## Description

Takes a block position and an axis vector and pushes the block along that vector. Costs an eighth of an [[Amethyst|Amethyst Dust]].

Similar to the block it is mimicking, this spell can push chains of up to 12 blocks if needed and break fragile blocks that a regular **Piston** would break. It also respects the stickiness of blocks like **Slime** and **Honey**, as well as the immovability of certain blocks like **Obsidian**.

## Related patterns (same book page)

- [[Dispense]] `item entity, vector, vector → —`
- [[Cook]] `item entity → —`
- [[Roast]] `item entity → —`
- [[Smoke]] `item entity → —`
- [[Blast]] `item entity → —`
- [[Cut Stone]] `item entity, identifier → —`

## Sources

- 📖 **Guidebook:** [[Block Mimicry]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPiston` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:piston`
