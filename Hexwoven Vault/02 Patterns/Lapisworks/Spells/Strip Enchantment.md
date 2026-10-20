---
aliases: ["Strip Enchantment"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/enchanting, topic/effects]
kind: "Spell"
family: "Spell"
topics: ["Enchanting & Imbuement", "Status Effects"]
mod: "Lapisworks"
op_id: "lapisworks:disenchant"
signature: "wedewqawwqqwqqw"
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

# Strip Enchantment

✦ **Spell** · [[Lapisworks]] · 📖 [[Enchantments]] · Lapisworks

**Topics:** [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]] · [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,54.7 76.9,85.0 94.4,115.3 76.9,145.5 59.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 111.8,85.0 94.4,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:559;--dur:4.5s" points="59.4,54.7 76.9,85.0 94.4,115.3 76.9,145.5 59.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 111.8,85.0 94.4,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="54.7" x2="65.0" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,71.4 61.7,66.3 68.3,62.5" fill="#fc77be"/><circle cx="59.4" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wedewqawwqqwqqw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, num` ([[Number]], [[Entity]])
- **Leaves:** `num` ([[Number]])
- **ID:** `lapisworks:disenchant`

```text
before   … │ entity │ num  ◂ top
after    … │ num           ◂ top
```

## Description

Reduces the level of the specified (with ID) enchantment on the given entity for one **Charged Amethyst**,. Cost is negated if there is nothing to remove.

## Related patterns (same book page)

- [[Enchantment Purification]] `entity, num → num`
- [[Enchant with FireyFists]] `entity → —`
- [[Enchant with Lightningbending]] `entity → —`
- [[Envelop Feet in Amel]] `entity → —`
- [[Grant Amel Air Sacs]] `entity → —`
- [[Make Fire-born]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Enchantments]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `Disenchant` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:disenchant`
