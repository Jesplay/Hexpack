---
aliases: ["Use Item On"]
tags: [hexcasting/pattern, mod/hexal, family/other, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/use_on"
signature: "qqqwqqqqaa"
start_direction: "EAST"
input: "mote, (entity | vec, vec)"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]", "[[Mote]]"]
iotas_out: []
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Use Item On

◇ **Pattern** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 65.7,60.5 85.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 65.7,60.5 85.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="94.0" x2="97.4" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="106.2,94.0 97.4,98.2 97.4,89.7" fill="#fc77be"/><circle cx="85.0" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqwqqqqaa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mote, (entity | vec, vec)` ([[Vector]], [[Entity]], [[Mote]])
- **Leaves:** `nothing` (—)
- **ID:** `hexal:mote/use_on`

## Description

Accepts a mote, and either an entity, or a target block and a normal vector. Uses the item on the entity/block. The mote must either only have one item in it, or the items contained must be extremely simple (no NBT data).

## Related patterns (same book page)

- [[Bind Storage]] `vec → —`
- [[Bind Storage - Temporary]] `vec → —`
- [[Stocktake Reflection]] `— → [itemtype]`
- [[Stocktake Purification]] `mote | itemtype → [mote]`
- [[Capacity Reflection]] `— → int`
- [[Containment Distillation]] `vec, (itemtype | mote) → bool`
- [[Mediafy Item]] `item | item, mote → mote`
- [[Return Item]] `mote, vec | mote, vec, num → —`
- [[Stacking Distillation II]] `mote, (mote | item) → bool`
- [[Splitting Gambit]] `mote, int → mote, mote`
- [[Depot Purification]] `mote → vec`
- [[Depot Gambit]] `mote, vec → mote`
- [[Craft]] `mote | [mote] | [[mote]] → [mote]`
- [[Preview Craft]] `item | [item] | [[item]] → [item]`
- [[Seniority Purification]] `villager → int`
- [[Offering Purification]] `villager → [complicated!]`
- [[Trade]] `villager, [mote], int? → item`

## Sources

- 📖 **Guidebook:** [[Motes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Use Item On](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/use_on)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpUseMoteOn` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/use_on`
