---
aliases: ["Conjure Mage Block"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:conjure_mage_block"
signature: "dee"
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

# Conjure Mage Block

✦ **Spell** · [[Hexical]] · 📖 [[Mage Blocks]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="65.1,85.0 38.0,38.0 92.3,38.0 119.4,85.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="65.1,85.0 38.0,38.0 92.3,38.0 119.4,85.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.1" y1="85.0" x2="56.4" y2="70.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.2,59.1 61.6,67.0 51.3,72.9" fill="#fc77be"/><circle cx="65.1" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about three Amethyst Dust.
- **ID:** `hexical:conjure_mage_block`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Conjures a **Mage Blocks** at the location. Costs about three [[Amethyst|Amethyst Dust]].

*I can create a city out of nothing, or I can cover it in volcanic rock.*

## Related patterns (same book page)

- [[Reset Mage Block]] `vector → —`
- [[Modify Block Bouncy]] `vector → —`
- [[Modify Block Energized]] `vector, number → —`
- [[Modify Block Ephemeral]] `vector, number → —`
- [[Modify Block Volatile]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Mage Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Mage Block](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/mage_blocks@hexical:conjure_mage_block)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureMageBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:conjure_mage_block`
