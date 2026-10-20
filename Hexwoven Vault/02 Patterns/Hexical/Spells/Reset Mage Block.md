---
aliases: ["Reset Mage Block"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:reset_mage_block"
signature: "deeeaw"
start_direction: "NORTH_WEST"
input: "vector"
output: ""
cost: "Costs about three Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Mage Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Reset Mage Block

✦ **Spell** · [[Hexical]] · 📖 [[Mage Blocks]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="101" height="170" viewBox="0 0 101.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,54.7 24.5,24.5 59.4,24.5 76.9,54.7 59.4,85.0 24.5,85.0 41.9,115.3 59.4,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:2.0s" points="41.9,54.7 24.5,24.5 59.4,24.5 76.9,54.7 59.4,85.0 24.5,85.0 41.9,115.3 59.4,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="54.7" x2="36.3" y2="45.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="32.3,38.1 39.7,43.1 33.0,47.0" fill="#fc77be"/><circle cx="41.9" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeeaw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about three Amethyst Dust.
- **ID:** `hexical:reset_mage_block`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Conjures a **Mage Blocks** at the location. Costs about three [[Amethyst|Amethyst Dust]].

*I can create a city out of nothing, or I can cover it in volcanic rock.*

## Related patterns (same book page)

- [[Conjure Mage Block]] `vector → —`
- [[Modify Block Bouncy]] `vector → —`
- [[Modify Block Energized]] `vector, number → —`
- [[Modify Block Ephemeral]] `vector, number → —`
- [[Modify Block Volatile]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Mage Blocks]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpResetMageBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:reset_mage_block`
