---
aliases: ["Thaumaturgist's Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing, topic/items]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts", "Items & Crafting"]
mod: "Hexpose"
op_id: "hexpose:get_enchantments"
signature: "waqeaeqawqwawaw"
start_direction: "WEST"
input: "item"
output: "list of identifiers"
cost: ""
iotas_in: ["[[Item Stack]]"]
iotas_out: ["[[List]]", "[[Identifier]]"]
entry: "[[Items (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Thaumaturgist's Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Items (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="85.0,22.0 53.5,22.0 22.0,22.0 37.8,49.3 69.3,49.3 85.0,76.6 100.7,49.3 132.2,49.3 148.0,22.0 116.5,22.0 85.0,22.0 69.3,49.3 53.5,76.6 85.0,76.6 116.5,76.6 100.7,49.3 85.0,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:504;--dur:4.5s" points="85.0,22.0 53.5,22.0 22.0,22.0 37.8,49.3 69.3,49.3 85.0,76.6 100.7,49.3 132.2,49.3 148.0,22.0 116.5,22.0 85.0,22.0 69.3,49.3 53.5,76.6 85.0,76.6 116.5,76.6 100.7,49.3 85.0,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="22.0" x2="74.9" y2="22.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.7,22.0 74.9,18.6 74.9,25.5" fill="#fc77be"/><circle cx="85.0" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqeaeqawqwawaw` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item` ([[Item Stack]])
- **Leaves:** `list of identifiers` ([[List]], [[Identifier]])
- **ID:** `hexpose:get_enchantments`

```text
before   … │ item                 ◂ top
after    … │ list of identifiers  ◂ top
```

## Description

Gets a list of enchantments the item has.

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
- [[Charm Distillation]] `item, identifier → number`
- [[Appellation Purification]] `item → text`
- [[Legacy Purification]] `item → list of text`
- [[Glamour Purification]] `item → number`
- [[Glamour Purification II]] `item → number`
- [[Reading Purification]] `item → list of text`
- [[Bibliography Purification]] `item → text, number`
- [[Collector Purification]] `item → number`

## Sources

- 📖 **Guidebook:** [[Items (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Thaumaturgist's Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/enchantments@hexpose:get_enchantments)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetItemStackData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_enchantments`
