---
aliases: ["Freeze"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexal"
op_id: "hexal:freeze"
signature: "weeeweedada"
start_direction: "WEST"
input: "vec"
output: ""
cost: "Costs one Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Hexal Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Freeze

✦ **Spell** · [[Hexal]] · 📖 [[Hexal Spells]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5 85.0,94.0 65.7,60.5 46.4,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5 85.0,94.0 65.7,60.5 46.4,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="94.0" x2="111.3" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="102.4,94.0 111.3,89.7 111.3,98.2" fill="#fc77be"/><circle cx="123.6" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weeeweedada` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs one Amethyst Dust.
- **ID:** `hexal:freeze`

```text
before   … │ vec  ◂ top
after    …        ◂ top
```

## Description

Freezes the block at a block location. For example, turns water into ice, and ice into packed ice. Costs one [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Smelt]] `vec | entity | mote → —`
- [[Falling Block]] `vec → —`
- [[Place Block II]] `(itemtype | mote), vec → —`
- [[Particles]] `vec | [vec] → —`

## Sources

- 📖 **Guidebook:** [[Hexal Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Freeze](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/hexal_spells@hexal:freeze)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpFreeze` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:freeze`
