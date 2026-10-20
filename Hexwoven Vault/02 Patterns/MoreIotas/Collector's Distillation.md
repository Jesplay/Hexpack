---
aliases: ["Collector's Distillation"]
tags: [hexcasting/pattern, mod/moreiotas, family/distillation, topic/items, topic/data-structures]
kind: "Pattern"
family: "Distillation"
topics: ["Items & Crafting", "Sets, Maps & Special Iotas"]
mod: "MoreIotas"
op_id: "moreiotas:item/inventory/items"
signature: "dewqa"
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

# Collector's Distillation

◇ **Pattern** · [[MoreIotas]] · 📖 [[Items]] · Patterns

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="84" height="170" viewBox="0 0 83.9 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,54.7 41.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3 41.9,145.5 59.4,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:210;--dur:1.7s" points="24.5,54.7 41.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3 41.9,145.5 59.4,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="54.7" x2="30.1" y2="45.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="34.1,38.1 33.4,47.0 26.7,43.1" fill="#fc77be"/><circle cx="24.5" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dewqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector` ([[Vector]])
- **Leaves:** `list` ([[List]])
- **ID:** `moreiotas:item/inventory/items`

```text
before   … │ vector │ vector  ◂ top
after    … │ list             ◂ top
```

## Description

Removes two vectors from the [[The Stack|stack]] representing position and side. Returns a list of Items in the block at the given position accessible from the given side (or all if second vector is zero).

## Related patterns (same book page)

- [[Duelist's Purification]] `entity → item stack`
- [[Shieldbearer's Purification]] `entity → item stack`
- [[Hoarder's Distillation]] `vector, vector → list`
- [[Treasurer's Distillation]] `itemtype|item stack, int → item stack`

## Sources

- 📖 **Guidebook:** [[Items]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Collector's Distillation](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/items@moreiotas:item/inventory/items)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetInventoryContents` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:item/inventory/items`
