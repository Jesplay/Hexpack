---
aliases: ["Modify Block: Volatile"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:modify_block_volatile"
signature: "deewedeeeee"
start_direction: "NORTH_WEST"
input: "vector"
output: ""
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Mage Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Modify Block: Volatile

✦ **Spell** · [[Hexical]] · 📖 [[Mage Blocks]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,54.7 41.9,24.5 76.9,24.5 94.4,54.7 76.9,85.0 59.4,115.3 24.5,115.3 41.9,85.0 76.9,85.0 94.4,115.3 76.9,145.5 41.9,145.5 24.5,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:419;--dur:3.4s" points="59.4,54.7 41.9,24.5 76.9,24.5 94.4,54.7 76.9,85.0 59.4,115.3 24.5,115.3 41.9,85.0 76.9,85.0 94.4,115.3 76.9,145.5 41.9,145.5 24.5,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="54.7" x2="53.8" y2="45.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.8,38.1 57.2,43.1 50.5,47.0" fill="#fc77be"/><circle cx="59.4" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deewedeeeee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:modify_block_volatile`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Breaking volatile **Mage Blocks** shatters others it is touching. This can cascade until every connected volatile block shatters.

## Related patterns (same book page)

- [[Conjure Mage Block]] `vector → —`
- [[Reset Mage Block]] `vector → —`
- [[Modify Block Bouncy]] `vector → —`
- [[Modify Block Energized]] `vector, number → —`
- [[Modify Block Ephemeral]] `vector, number → —`

## Sources

- 📖 **Guidebook:** [[Mage Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Modify Block: Volatile](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/mage_blocks@hexical:modify_block_volatile)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConfigureMageBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:modify_block_volatile`
