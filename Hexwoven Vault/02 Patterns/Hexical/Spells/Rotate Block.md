---
aliases: ["Rotate Block"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:rotate_block"
signature: "edeeeeeweewadeeed"
start_direction: "EAST"
input: "vector, vector"
output: ""
cost: "Costs about an eighth of an Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Rotation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Rotate Block

✦ **Spell** · [[Hexical]] · 📖 [[Rotation]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/><circle cx="22.0" cy="103.8" r="2"/><circle cx="53.5" cy="103.8" r="2"/><circle cx="85.0" cy="103.8" r="2"/><circle cx="116.5" cy="103.8" r="2"/><circle cx="148.0" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="85.0,49.3 116.5,49.3 132.2,76.6 100.7,76.6 85.0,49.3 100.7,22.0 132.2,22.0 148.0,49.3 132.2,76.6 116.5,103.8 85.0,103.8 69.3,76.6 53.5,49.3 37.8,76.6 22.0,49.3 37.8,22.0 69.3,22.0 85.0,49.3 53.5,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:5.0s" points="85.0,49.3 116.5,49.3 132.2,76.6 100.7,76.6 85.0,49.3 100.7,22.0 132.2,22.0 148.0,49.3 132.2,76.6 116.5,103.8 85.0,103.8 69.3,76.6 53.5,49.3 37.8,76.6 22.0,49.3 37.8,22.0 69.3,22.0 85.0,49.3 53.5,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="49.3" x2="95.1" y2="49.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="102.3,49.3 95.1,52.8 95.1,45.8" fill="#fc77be"/><circle cx="85.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeeeeeweewadeeed` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about an eighth of an Amethyst Dust.
- **ID:** `hexical:rotate_block`

```text
before   … │ vector │ vector  ◂ top
after    …                    ◂ top
```

## Description

Rotates a block to face a certain vector direction. Requires a rotatable block. Costs about an eighth of an [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Rotate Entity]] `entity, number, number → —`

## Sources

- 📖 **Guidebook:** [[Rotation]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpRotateBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:rotate_block`
