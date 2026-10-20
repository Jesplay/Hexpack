---
aliases: ["Aegis Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing, topic/items]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts", "Items & Crafting"]
mod: "Hexpose"
op_id: "hexpose:get_armor"
signature: "qaqddqeeeeqd"
start_direction: "NORTH_EAST"
input: "entity"
output: "list of items"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[List]]", "[[Item Stack]]"]
entry: "[[Items (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Aegis Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Items (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,140.2 93.5,103.4 72.3,66.6 51.0,103.4 72.3,140.2 29.8,140.2 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4 114.8,140.2 72.3,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:553;--dur:3.6s" points="72.3,140.2 93.5,103.4 72.3,66.6 51.0,103.4 72.3,140.2 29.8,140.2 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4 114.8,140.2 72.3,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="140.2" x2="79.1" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="84.0,120.0 83.1,130.8 75.0,126.1" fill="#fc77be"/><circle cx="72.3" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqddqeeeeqd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `list of items` ([[List]], [[Item Stack]])
- **ID:** `hexpose:get_armor`

```text
before   … │ entity         ◂ top
after    … │ list of items  ◂ top
```

## Description

Returns a list of all items the entity is wearing, from feet up.

## Related patterns (same book page)

- [[Item Purification]] `entity → item`
- [[Offer Purification]] `identifier, number → item`
- [[Tool Purification]] `entity → item`
- [[Accessory Purification]] `entity → item`
- [[Cart Purification]] `entity → list of items | null`
- [[Chest Purification]] `vector → list of items | null`
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
- [[Bibliography Purification]] `item → text, number`
- [[Collector Purification]] `item → number`

## Sources

- 📖 **Guidebook:** [[Items (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Aegis Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/item@hexpose:get_armor)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetArmor` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_armor`
