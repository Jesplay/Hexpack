---
aliases: ["Stacking Distillation II"]
tags: [hexcasting/pattern, mod/hexal, family/distillation, topic/items]
kind: "Pattern"
family: "Distillation"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/combinable"
signature: "dedqeaqa"
start_direction: "SOUTH_WEST"
input: "mote, (mote | item)"
output: "bool"
cost: ""
iotas_in: ["[[Item Stack]]", "[[Mote]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Stacking Distillation II

◇ **Pattern** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="65.7,60.5 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="59.5" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,78.9 55.8,69.1 63.2,73.3" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedqeaqa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mote, (mote | item)` ([[Item Stack]], [[Mote]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexal:mote/combinable`

## Description

Returns true if the first mote can be combined with the items in the second argument (which can be a mote, item [[The Stack|stack]], item entity, or item frame), and false otherwise.

## Related patterns (same book page)

- [[Bind Storage]] `vec → —`
- [[Bind Storage - Temporary]] `vec → —`
- [[Stocktake Reflection]] `— → [itemtype]`
- [[Stocktake Purification]] `mote | itemtype → [mote]`
- [[Capacity Reflection]] `— → int`
- [[Containment Distillation]] `vec, (itemtype | mote) → bool`
- [[Mediafy Item]] `item | item, mote → mote`
- [[Return Item]] `mote, vec | mote, vec, num → —`
- [[Splitting Gambit]] `mote, int → mote, mote`
- [[Depot Purification]] `mote → vec`
- [[Depot Gambit]] `mote, vec → mote`
- [[Craft]] `mote | [mote] | [[mote]] → [mote]`
- [[Preview Craft]] `item | [item] | [[item]] → [item]`
- [[Seniority Purification]] `villager → int`
- [[Offering Purification]] `villager → [complicated!]`
- [[Trade]] `villager, [mote], int? → item`
- [[Use Item On]] `mote, (entity | vec, vec) → —`

## Sources

- 📖 **Guidebook:** [[Motes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Stacking Distillation II](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/combinable)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMotesCombinable` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/combinable`
