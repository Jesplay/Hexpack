---
aliases: ["Shieldbearer's Purification"]
tags: [hexcasting/pattern, mod/moreiotas, family/purification, topic/items, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["Items & Crafting", "Sets, Maps & Special Iotas"]
mod: "MoreIotas"
op_id: "moreiotas:item/off_hand"
signature: "qeda"
start_direction: "EAST"
input: "entity"
output: "item stack"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Item Stack]]"]
entry: "[[Items]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Shieldbearer's Purification

◇ **Pattern** · [[MoreIotas]] · 📖 [[Items]] · Patterns

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="88" viewBox="0 0 170.0 87.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 85.0,27.0 123.6,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:193;--dur:1.6s" points="27.0,60.5 65.7,60.5 85.0,27.0 123.6,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qeda` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `item stack` ([[Item Stack]])
- **ID:** `moreiotas:item/off_hand`

```text
before   … │ entity      ◂ top
after    … │ item stack  ◂ top
```

## Description

Transforms the entity at the top of the [[The Stack|stack]] into the Item Stack in its off hand.

## Related patterns (same book page)

- [[Duelist's Purification]] `entity → item stack`
- [[Hoarder's Distillation]] `vector, vector → list`
- [[Collector's Distillation]] `vector, vector → list`
- [[Treasurer's Distillation]] `itemtype|item stack, int → item stack`

## Sources

- 📖 **Guidebook:** [[Items]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Shieldbearer's Purification](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/items@moreiotas:item/off_hand)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetHeldItem` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:item/off_hand`
