---
aliases: ["Cut Stone"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:stonecut"
signature: "qqqqqwaeaeaeaeaeadawa"
start_direction: "EAST"
input: "item entity, identifier"
output: ""
cost: "Costs about an eighth of one Amethyst Dust."
iotas_in: ["[[Entity]]", "[[Identifier]]"]
iotas_out: []
entry: "[[Block Mimicry]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Cut Stone

✦ **Spell** · [[Hexical]] · 📖 [[Block Mimicry]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,115.3 94.4,115.3 111.8,85.0 94.4,54.7 59.4,54.7 41.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 41.9,145.5 76.9,145.5 111.8,145.5 94.4,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:769;--dur:6.0s" points="59.4,115.3 94.4,115.3 111.8,85.0 94.4,54.7 59.4,54.7 41.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 41.9,145.5 76.9,145.5 111.8,145.5 94.4,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="115.3" x2="70.6" y2="115.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="78.6,115.3 70.6,119.1 70.6,111.4" fill="#fc77be"/><circle cx="59.4" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqwaeaeaeaeaeadawa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item entity, identifier` ([[Entity]], [[Identifier]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about an eighth of one Amethyst Dust.
- **ID:** `hexical:stonecut`

```text
before   … │ item entity │ identifier  ◂ top
after    …                             ◂ top
```

## Description

Takes an item entity and cuts the item into the shape of an item identifier. Costs about an eighth of one [[Amethyst|Amethyst Dust]].

The target item must be possible to carve from the source item in a **Stonecutter**. For instance, **Stone** can be cut into all manners of stairs, slabs, and walls.

*I saw the angel in the marble and carved until I set him free.*

## Related patterns (same book page)

- [[Push]] `vector, vector → —`
- [[Dispense]] `item entity, vector, vector → —`
- [[Cook]] `item entity → —`
- [[Roast]] `item entity → —`
- [[Smoke]] `item entity → —`
- [[Blast]] `item entity → —`

## Sources

- 📖 **Guidebook:** [[Block Mimicry]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Cut Stone](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/block_mimicry@hexical:stonecut)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpStonecut` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:stonecut`
