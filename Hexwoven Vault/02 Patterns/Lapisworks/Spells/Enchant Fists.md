---
aliases: ["Enchant Fists"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/enchanting]
kind: "Spell"
family: "Spell"
topics: ["Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:moar_attack"
signature: "qaqwweaeaqwww"
start_direction: "EAST"
input: "entity, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Lapisworks Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Enchant Fists

✦ **Spell** · [[Lapisworks]] · 📖 [[Lapisworks Patterns]] · Lapisworks

**Topics:** [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 145.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="139.4" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="139.4" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/><circle cx="139.4" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="80.1,46.5 109.8,46.5 124.6,20.8 94.9,20.8 80.1,46.5 65.3,72.2 50.4,97.8 20.8,97.8 35.6,123.5 20.8,149.2 50.4,149.2 65.3,123.5 80.1,97.8 94.9,72.2 109.8,46.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:415;--dur:3.9s" points="80.1,46.5 109.8,46.5 124.6,20.8 94.9,20.8 80.1,46.5 65.3,72.2 50.4,97.8 20.8,97.8 35.6,123.5 20.8,149.2 50.4,149.2 65.3,123.5 80.1,97.8 94.9,72.2 109.8,46.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="80.1" y1="46.5" x2="89.6" y2="46.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="96.4,46.5 89.6,49.7 89.6,43.2" fill="#fc77be"/><circle cx="80.1" cy="46.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqwweaeaqwww` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, num` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:moar_attack`

```text
before   … │ entity │ num  ◂ top
after    …                 ◂ top
```

## Description

Pass in an entity and a number of half-hearts, and it enchants their fists to increase their damage per hit. One **Amel** and five **Amethyst Shards** per half-heart. Limit is 4x the norm so just 4 for me.

## Related patterns (same book page)

- [[Imbue Lapis]] `— → —`
- [[Reclaim Amethyst]] `— → —`
- [[Mold Amel]] `— → —`
- [[Enchant Skin]] `entity, num → —`
- [[Enchant Feet]] `entity, num → —`
- [[Enchant Arms]] `entity, bool → —`
- [[Enhancement Purification]] `entity, num → —`

## Sources

- 📖 **Guidebook:** [[Lapisworks Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `?` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:moar_attack`
