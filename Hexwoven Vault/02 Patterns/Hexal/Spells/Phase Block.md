---
aliases: ["Phase Block"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexal"
op_id: "hexal:interop/fabric_only/phase_block"
signature: "daqqqa"
start_direction: "WEST"
input: "vec, num"
output: ""
cost: "Costs one ten-thousandth of an Amethyst Dust times the square of the time to phase out for."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Fabric-Only Hexal Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Phase Block

✦ **Spell** · [[Hexal]] · 📖 [[Fabric-Only Hexal Spells]] · Cross-Mod Compatibility

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="135.0,78.3 85.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:2.0s" points="135.0,78.3 85.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="135.0" y1="78.3" x2="119.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.5,78.3 119.0,72.8 119.0,83.8" fill="#fc77be"/><circle cx="135.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `daqqqa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs one ten-thousandth of an Amethyst Dust times the square of the time to phase out for.
- **ID:** `hexal:interop/fabric_only/phase_block`

```text
before   … │ vec │ num  ◂ top
after    …              ◂ top
```

## Description

Takes a position and a number, and phases the block at that position out for that many seconds. Costs one ten-thousandth of an [[Amethyst|Amethyst Dust]] times the square of the time to phase out for.

## Sources

- 📖 **Guidebook:** [[Fabric-Only Hexal Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Phase Block](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#interop/fabric_only_hexal@hexal:interop/fabric_only/phase_block)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPhaseBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:interop/fabric_only/phase_block`
