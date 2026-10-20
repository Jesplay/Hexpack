---
aliases: ["Hoarder's Distillation"]
tags: [hexcasting/pattern, mod/moreiotas, family/distillation, topic/items, topic/data-structures]
kind: "Pattern"
family: "Distillation"
topics: ["Items & Crafting", "Sets, Maps & Special Iotas"]
mod: "MoreIotas"
op_id: "moreiotas:item/inventory/stacks"
signature: "aqwed"
start_direction: "NORTH_EAST"
input: "vector, vector"
output: "list"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[List]]"]
entry: "[[Items]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Hoarder's Distillation

◇ **Pattern** · [[MoreIotas]] · 📖 [[Items]] · Patterns

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="123.6,60.5 143.0,27.0 104.3,27.0 85.0,60.5 65.7,94.0 27.0,94.0 46.4,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:232;--dur:1.7s" points="123.6,60.5 143.0,27.0 104.3,27.0 85.0,60.5 65.7,94.0 27.0,94.0 46.4,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="60.5" x2="129.8" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="134.3,42.1 133.5,51.9 126.1,47.7" fill="#fc77be"/><circle cx="123.6" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqwed` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector` ([[Vector]])
- **Leaves:** `list` ([[List]])
- **ID:** `moreiotas:item/inventory/stacks`

```text
before   … │ vector │ vector  ◂ top
after    … │ list             ◂ top
```

## Description

Removes two vectors from the [[The Stack|stack]] representing position and side. Returns a list of Item Stacks in the block at the given position accessible from the given side (or all if second vector is zero).

## Related patterns (same book page)

- [[Duelist's Purification]] `entity → item stack`
- [[Shieldbearer's Purification]] `entity → item stack`
- [[Collector's Distillation]] `vector, vector → list`
- [[Treasurer's Distillation]] `itemtype|item stack, int → item stack`

## Sources

- 📖 **Guidebook:** [[Items]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Hoarder's Distillation](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/items@moreiotas:item/inventory/stacks)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetInventoryContents` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:item/inventory/stacks`
