---
aliases: ["Enchantment Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/enchanting, topic/effects]
kind: "Pattern"
family: "Purification"
topics: ["Enchanting & Imbuement", "Status Effects"]
mod: "Lapisworks"
op_id: "lapisworks:checkenchant"
signature: "aqawwqqwqqw"
start_direction: "SOUTH_EAST"
input: "entity, num"
output: "num"
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Enchantments]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Enchantment Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Enchantments]] · Lapisworks

**Topics:** [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]] · [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,54.7 76.9,85.0 94.4,54.7 76.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 111.8,85.0 94.4,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:419;--dur:3.4s" points="59.4,54.7 76.9,85.0 94.4,54.7 76.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 111.8,85.0 94.4,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="54.7" x2="65.0" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,71.4 61.7,66.3 68.3,62.5" fill="#fc77be"/><circle cx="59.4" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqawwqqwqqw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, num` ([[Number]], [[Entity]])
- **Leaves:** `num` ([[Number]])
- **ID:** `lapisworks:checkenchant`

```text
before   … │ entity │ num  ◂ top
after    … │ num           ◂ top
```

## Description

Gives the level of the specified (with ID) enchantment on the given entity for little cost .

## Related patterns (same book page)

- [[Strip Enchantment]] `entity, num → num`
- [[Enchant with FireyFists]] `entity → —`
- [[Enchant with Lightningbending]] `entity → —`
- [[Envelop Feet in Amel]] `entity → —`
- [[Grant Amel Air Sacs]] `entity → —`
- [[Make Fire-born]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Enchantments]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `CheckEnchant` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:checkenchant`
