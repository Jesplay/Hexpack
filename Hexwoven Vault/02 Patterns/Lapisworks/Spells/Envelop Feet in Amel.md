---
aliases: ["Envelop Feet in Amel"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/enchanting, topic/effects]
kind: "Spell"
family: "Spell"
topics: ["Enchanting & Imbuement", "Status Effects"]
mod: "Lapisworks"
op_id: "lapisworks:falldmgres"
signature: "qqwwqqqadwewdeq"
start_direction: "SOUTH_WEST"
input: "entity"
output: ""
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Enchantments]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Envelop Feet in Amel

✦ **Spell** · [[Lapisworks]] · 📖 [[Enchantments]] · Lapisworks

**Topics:** [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]] · [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="37.8,49.3 22.0,76.6 37.8,103.8 69.3,103.8 100.7,103.8 132.2,103.8 148.0,76.6 132.2,49.3 100.7,49.3 116.5,76.6 85.0,76.6 53.5,76.6 37.8,49.3 22.0,22.0 53.5,22.0 69.3,49.3 100.7,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:504;--dur:4.5s" points="37.8,49.3 22.0,76.6 37.8,103.8 69.3,103.8 100.7,103.8 132.2,103.8 148.0,76.6 132.2,49.3 100.7,49.3 116.5,76.6 85.0,76.6 53.5,76.6 37.8,49.3 22.0,22.0 53.5,22.0 69.3,49.3 100.7,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="37.8" y1="49.3" x2="32.7" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="29.1,64.3 29.7,56.3 35.7,59.8" fill="#fc77be"/><circle cx="37.8" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqwwqqqadwewdeq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:falldmgres`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

Cushions up to 20 blocks of fall damage per level, costing 5 **Charged Amethyst** and 20 **Amel** per level. Three levels. ID = 2.

## Related patterns (same book page)

- [[Enchantment Purification]] `entity, num → num`
- [[Strip Enchantment]] `entity, num → num`
- [[Enchant with FireyFists]] `entity → —`
- [[Enchant with Lightningbending]] `entity → —`
- [[Grant Amel Air Sacs]] `entity → —`
- [[Make Fire-born]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Enchantments]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `?` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:falldmgres`
