---
aliases: ["Bibliography Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/text, topic/items]
kind: "Pattern"
family: "Purification"
topics: ["Text, Strings & Chat", "Items & Crafting"]
mod: "Hexpose"
op_id: "hexpose:book_sources"
signature: "eaedweew"
start_direction: "EAST"
input: "item"
output: "text, number"
cost: ""
iotas_in: ["[[Item Stack]]"]
iotas_out: ["[[Number]]", "[[String]]"]
entry: "[[Items (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Bibliography Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Items (Hexpose)]] · Patterns › Scrying

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,27.0 65.7,27.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 104.3,94.0 65.7,94.0 46.4,60.5 27.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="27.0,27.0 65.7,27.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 104.3,94.0 65.7,94.0 46.4,60.5 27.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="27.0" x2="39.4" y2="27.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,27.0 39.4,31.3 39.4,22.8" fill="#fc77be"/><circle cx="27.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaedweew` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item` ([[Item Stack]])
- **Leaves:** `text, number` ([[Number]], [[String]])
- **ID:** `hexpose:book_sources`

```text
before   … │ item           ◂ top
after    … │ text │ number  ◂ top
```

## Description

Pushes the author of a book and its generation number.

## Related patterns (same book page)

- [[Item Purification]] `entity → item`
- [[Offer Purification]] `identifier, number → item`
- [[Tool Purification]] `entity → item`
- [[Accessory Purification]] `entity → item`
- [[Cart Purification]] `entity → list of items | null`
- [[Chest Purification]] `vector → list of items | null`
- [[Aegis Purification]] `entity → list of items`
- [[Pocket Reflection]] `— → list of items`
- [[Storage Purification]] `item → number`
- [[Warehouse Purification]] `identifier → number`
- [[Deterioration Purification]] `item → number`
- [[Fragility Purification]] `identifier → number`
- [[Thaumaturgist's Purification]] `item → list of identifiers`
- [[Charm Distillation]] `item, identifier → number`
- [[Appellation Purification]] `item → text`
- [[Legacy Purification]] `item → list of text`
- [[Glamour Purification]] `item → number`
- [[Glamour Purification II]] `item → number`
- [[Reading Purification]] `item → list of text`
- [[Collector Purification]] `item → number`

## Sources

- 📖 **Guidebook:** [[Items (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Bibliography Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/item@hexpose:book_sources)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpBookSources` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:book_sources`
