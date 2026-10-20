---
aliases: ["Capacity Reflection"]
tags: [hexcasting/pattern, mod/hexal, family/reflection, topic/items]
kind: "Pattern"
family: "Reflection"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/storage/remaining_capacity/get"
signature: "awedqdewa"
start_direction: "SOUTH_EAST"
input: ""
output: "int"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Capacity Reflection

◇ **Pattern** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,68.3 52.3,106.0 74.1,68.3 95.9,30.5 139.5,30.5 117.7,68.3 139.5,106.0 95.9,106.0 74.1,68.3 52.3,30.5 30.5,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:436;--dur:2.8s" points="30.5,68.3 52.3,106.0 74.1,68.3 95.9,30.5 139.5,30.5 117.7,68.3 139.5,106.0 95.9,106.0 74.1,68.3 52.3,30.5 30.5,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="68.3" x2="37.5" y2="80.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,89.0 33.3,82.7 41.6,77.9" fill="#fc77be"/><circle cx="30.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awedqdewa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `int` ([[Number]])
- **ID:** `hexal:mote/storage/remaining_capacity/get`

```text
before   …        ◂ top
after    … │ int  ◂ top
```

## Description

Gets the remaining capacity of the bound Mote Nexus.

## Related patterns (same book page)

- [[Bind Storage]] `vec → —`
- [[Bind Storage - Temporary]] `vec → —`
- [[Stocktake Reflection]] `— → [itemtype]`
- [[Stocktake Purification]] `mote | itemtype → [mote]`
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
- 🌐 **Official online docs:** [Capacity Reflection](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/storage/remaining_capacity/get)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetStorageRemainingCapacity` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/storage/remaining_capacity/get`
