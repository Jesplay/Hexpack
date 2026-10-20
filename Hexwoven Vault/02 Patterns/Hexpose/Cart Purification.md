---
aliases: ["Cart Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing, topic/items]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts", "Items & Crafting"]
mod: "Hexpose"
op_id: "hexpose:get_inventory"
signature: "edeeeeeqdee"
start_direction: "WEST"
input: "entity"
output: "list of items | null"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[List]]", "[[Null]]", "[[Item Stack]]"]
entry: "[[Items (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Cart Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Items (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="114.8,103.4 72.3,103.4 51.0,66.6 93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 29.8,29.8 72.3,29.8 93.5,66.6 72.3,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:510;--dur:3.4s" points="114.8,103.4 72.3,103.4 51.0,66.6 93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 29.8,29.8 72.3,29.8 93.5,66.6 72.3,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="114.8" y1="103.4" x2="101.2" y2="103.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="91.4,103.4 101.2,98.7 101.2,108.1" fill="#fc77be"/><circle cx="114.8" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeeeeeqdee` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `list of items | null` ([[List]], [[Null]], [[Item Stack]])
- **ID:** `hexpose:get_inventory`

## Description

Returns a list of all items the entity is carrying, or Null if the entity can not hold items. Trying to access another **Player**'s inventory will incur a [[Mishaps|mishap]].

## Related patterns (same book page)

- [[Item Purification]] `entity → item`
- [[Offer Purification]] `identifier, number → item`
- [[Tool Purification]] `entity → item`
- [[Accessory Purification]] `entity → item`
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
- [[Bibliography Purification]] `item → text, number`
- [[Collector Purification]] `item → number`

## Sources

- 📖 **Guidebook:** [[Items (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Cart Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/item@hexpose:get_inventory)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetInventory` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_inventory`
