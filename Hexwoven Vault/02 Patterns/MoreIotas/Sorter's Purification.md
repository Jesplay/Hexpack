---
aliases: ["Sorter's Purification"]
tags: [hexcasting/pattern, mod/moreiotas, family/purification, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["Sets, Maps & Special Iotas"]
mod: "MoreIotas"
op_id: "moreiotas:type/to_item"
signature: "qaqqaea"
start_direction: "EAST"
input: "itemtypable"
output: "itemtype"
cost: ""
iotas_in: []
iotas_out: ["[[Item Type]]"]
entry: "[[Types (MoreIotas)]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Sorter's Purification

◇ **Pattern** · [[MoreIotas]] · 📖 [[Types (MoreIotas)]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 85.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 85.0,78.3 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="35.0,78.3 85.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 85.0,78.3 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="51.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,78.3 51.0,83.8 51.0,72.8" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqaea` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `itemtypable` (—)
- **Leaves:** `itemtype` ([[Item Type]])
- **ID:** `moreiotas:type/to_item`

```text
before   … │ itemtypable  ◂ top
after    … │ itemtype     ◂ top
```

## Description

Remove an [[Iotas|iota]] with an associated item type (for example a location and the type of the block at that location, an item entity, or an item frame) from the [[The Stack|stack]] and returns the item type to the stack (e.g. Grass, Stone, Stick, etc.).

## Other uses (same pattern, other inputs)

### [[Hexal]] · 📖 [[Motes]]

`mote` → `item stack`

Accepts a mote and returns an [[Item Iotas|Item Stack]] representing the items in the record referenced by that mote.

### [[MoreIotas]] · 📖 [[Items]]

`item stack` → `itemtype`

Transforms the [[Item Iotas|Item Stack]] at the top of the stack into its Item.

## Related patterns (same book page)

- [[Classifier's Purification]] `any → iotatype`
- [[Physician's Purification]] `entity → entitytype`
- [[Sorter's Reflection]] `— → itemtype`
- [[Entity Distillation Type]] `entitytype, pos → entity | null`
- [[Zone Exaltation Type]] `entitytype, pos, num → [entity]`
- [[Zone Exaltation Not-Type]] `entitytype, pos, num → [entity]`

## Sources

- 📖 **Guidebook:** [[Types (MoreIotas)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Sorter's Purification](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/types@moreiotas:type/to_item)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `moreiotas:type/to_item`
