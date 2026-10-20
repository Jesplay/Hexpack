---
aliases: ["Treasurer's Distillation"]
tags: [hexcasting/pattern, mod/moreiotas, family/distillation, topic/items, topic/data-structures]
kind: "Pattern"
family: "Distillation"
topics: ["Items & Crafting", "Sets, Maps & Special Iotas"]
mod: "MoreIotas"
op_id: "moreiotas:item/make"
signature: "adeeedew"
start_direction: "EAST"
input: "itemtype|item stack, int"
output: "item stack"
cost: ""
iotas_in: ["[[Number]]", "[[Item Type]]", "[[Item Stack]]"]
iotas_out: ["[[Item Stack]]"]
entry: "[[Items]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Treasurer's Distillation

◇ **Pattern** · [[MoreIotas]] · 📖 [[Items]] · Patterns

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 46.4,94.0 65.7,60.5 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="27.0,60.5 65.7,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 46.4,94.0 65.7,60.5 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adeeedew` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `itemtype|item stack, int` ([[Number]], [[Item Type]], [[Item Stack]])
- **Leaves:** `item stack` ([[Item Stack]])
- **ID:** `moreiotas:item/make`

## Description

Removes a number and either an itemtype or an Item [[The Stack|Stack]] from the stack, then creates a new Item Stack with the specified item and size.

## Related patterns (same book page)

- [[Duelist's Purification]] `entity → item stack`
- [[Shieldbearer's Purification]] `entity → item stack`
- [[Hoarder's Distillation]] `vector, vector → list`
- [[Collector's Distillation]] `vector, vector → list`

## Sources

- 📖 **Guidebook:** [[Items]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Treasurer's Distillation](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/items@moreiotas:item/make)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `moreiotas:item/make`
