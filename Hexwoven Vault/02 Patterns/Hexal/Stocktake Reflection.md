---
aliases: ["Stocktake Reflection"]
tags: [hexcasting/pattern, mod/hexal, family/reflection, topic/items]
kind: "Pattern"
family: "Reflection"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/contained_type/get"
signature: "dwqqqqqwddww"
start_direction: "NORTH_EAST"
input: ""
output: "[itemtype]"
cost: ""
iotas_in: []
iotas_out: ["[[List]]", "[[Item Type]]"]
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Stocktake Reflection

◇ **Pattern** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 46.4,27.0 65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 46.4,94.0 27.0,60.5 65.7,60.5 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:502;--dur:3.6s" points="27.0,60.5 46.4,27.0 65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 46.4,94.0 27.0,60.5 65.7,60.5 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="33.2" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,42.1 36.9,51.9 29.5,47.7" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwqqqqqwddww` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `[itemtype]` ([[List]], [[Item Type]])
- **ID:** `hexal:mote/contained_type/get`

```text
before   …               ◂ top
after    … │ [itemtype]  ◂ top
```

## Description

Gets all the types of motes contained in the currently bound Mote Nexus.

## Related patterns (same book page)

- [[Bind Storage]] `vec → —`
- [[Bind Storage - Temporary]] `vec → —`
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
- [[Use Item On]] `mote, (entity | vec, vec) → —`

## Sources

- 📖 **Guidebook:** [[Motes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Stocktake Reflection](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/contained_type/get)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetContainedItemTypes` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/contained_type/get`
