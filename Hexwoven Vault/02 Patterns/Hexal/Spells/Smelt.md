---
aliases: ["Smelt"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:smelt"
signature: "wqqqwqqadad"
start_direction: "EAST"
input: "vec | entity | mote"
output: ""
cost: "Costs three quarters of an Amethyst Dust per item smelted (i."
iotas_in: ["[[Vector]]", "[[Entity]]", "[[Mote]]"]
iotas_out: []
entry: "[[Hexal Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Smelt

✦ **Spell** · [[Hexal]] · 📖 [[Hexal Spells]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5 85.0,94.0 104.3,60.5 123.6,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5 85.0,94.0 104.3,60.5 123.6,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="94.0" x2="58.7" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.6,94.0 58.7,98.2 58.7,89.7" fill="#fc77be"/><circle cx="46.4" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqqwqqadad` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec | entity | mote` ([[Vector]], [[Entity]], [[Mote]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs three quarters of an Amethyst Dust per item smelted (i.
- **ID:** `hexal:smelt`

## Description

Remove a location, item entity, or mote from the [[The Stack|stack]], then either smelt the block at the location or smelt all the items in the item entity or mote.

Costs three quarters of an [[Amethyst|Amethyst Dust]] per item smelted (i.e. three quarters of an [[Amethyst|Amethyst Dust]] for smelting a block, 6 [[Amethyst|Amethyst Dust]] to smelt a stack of 8 items.)

## Related patterns (same book page)

- [[Freeze]] `vec → —`
- [[Falling Block]] `vec → —`
- [[Place Block II]] `(itemtype | mote), vec → —`
- [[Particles]] `vec | [vec] → —`

## Sources

- 📖 **Guidebook:** [[Hexal Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Smelt](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/hexal_spells@hexal:smelt)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSmelt` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:smelt`
