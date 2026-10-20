---
aliases: ["Place Block II"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexal"
op_id: "hexal:place_type"
signature: "eeeeedeeeee"
start_direction: "WEST"
input: "(itemtype | mote), vec"
output: ""
cost: "Costs an eighth of an Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Item Type]]", "[[Mote]]"]
iotas_out: []
entry: "[[Hexal Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Place Block II

✦ **Spell** · [[Hexal]] · 📖 [[Hexal Spells]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 65.7,60.5 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 65.7,60.5 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="94.0" x2="72.6" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="63.7,94.0 72.6,89.7 72.6,98.2" fill="#fc77be"/><circle cx="85.0" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeedeeeee` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `(itemtype | mote), vec` ([[Vector]], [[Item Type]], [[Mote]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs an eighth of an Amethyst Dust.
- **ID:** `hexal:place_type`

## Description

Accepts an item type or mote and a location, and attempts to place a block of that item type at that location. If a mote was passed it will instead place from the record referenced by that mote. Costs an eighth of an [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Smelt]] `vec | entity | mote → —`
- [[Freeze]] `vec → —`
- [[Falling Block]] `vec → —`
- [[Particles]] `vec | [vec] → —`

## Sources

- 📖 **Guidebook:** [[Hexal Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Place Block II](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/hexal_spells@hexal:place_type)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPlaceType` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:place_type`
