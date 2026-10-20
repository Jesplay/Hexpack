---
aliases: ["Dispense"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:dispense"
signature: "wqwawqwddaeeead"
start_direction: "SOUTH_WEST"
input: "item entity, vector, vector"
output: ""
cost: "costs about half of one Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Block Mimicry]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Dispense

✦ **Spell** · [[Hexical]] · 📖 [[Block Mimicry]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="146.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="146.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 59.4,24.5 94.4,24.5 76.9,54.7 111.8,54.7 129.3,85.0 111.8,115.3 76.9,115.3 94.4,145.5 59.4,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:559;--dur:4.5s" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 59.4,24.5 94.4,24.5 76.9,54.7 111.8,54.7 129.3,85.0 111.8,115.3 76.9,115.3 94.4,145.5 59.4,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="24.5" x2="53.8" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.8,41.1 50.5,32.2 57.2,36.1" fill="#fc77be"/><circle cx="59.4" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwawqwddaeeead` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item entity, vector, vector` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** costs about half of one Amethyst Dust.
- **ID:** `hexical:dispense`

```text
before   … │ item entity │ vector │ vector  ◂ top
after    …                                  ◂ top
```

## Description

Takes an item entity, a position, and an axis vector and dispenses the item. For most applications, costs about half of one [[Amethyst|Amethyst Dust]].

More specifically, this spell acts as though a dispenser suddenly appears with the item loaded and fires the dispenser.

If this spell is used for any type of **Arrow**, the price increases to a full [[Amethyst|Charged Amethyst]]. I wonder why Nature would have such an adverse reaction to this specific application. Perhaps I would be too powerful?

*Upon request, I can also perform a hug.*

## Related patterns (same book page)

- [[Push]] `vector, vector → —`
- [[Cook]] `item entity → —`
- [[Roast]] `item entity → —`
- [[Smoke]] `item entity → —`
- [[Blast]] `item entity → —`
- [[Cut Stone]] `item entity, identifier → —`

## Sources

- 📖 **Guidebook:** [[Block Mimicry]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Dispense](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/block_mimicry@hexical:dispense)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpDispense` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:dispense`
