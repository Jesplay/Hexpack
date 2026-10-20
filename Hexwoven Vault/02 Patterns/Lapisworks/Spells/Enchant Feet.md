---
aliases: ["Enchant Feet"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/enchanting]
kind: "Spell"
family: "Spell"
topics: ["Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:moar_speed"
signature: "ddqwaqeqa"
start_direction: "WEST"
input: "entity, num"
output: ""
cost: "Costs 1x Amel and 5x Amethyst Shards."
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Lapisworks Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Enchant Feet

✦ **Spell** · [[Lapisworks]] · 📖 [[Lapisworks Patterns]] · Lapisworks

**Topics:** [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,94.0 27.0,94.0 46.4,60.5 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="65.7,94.0 27.0,94.0 46.4,60.5 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="94.0" x2="53.3" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.4,94.0 53.3,89.7 53.3,98.2" fill="#fc77be"/><circle cx="65.7" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddqwaqeqa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, num` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs 1x Amel and 5x Amethyst Shards.
- **ID:** `lapisworks:moar_speed`

```text
before   … │ entity │ num  ◂ top
after    …                 ◂ top
```

## Description

Enchants the given entity's feet to increase their movement speed. Costs 1x **Amel** and 5x **Amethyst Shards**. Limit is 3x the norm. The norm for me is about 1.

This speed is unwieldy for myself, so enchanting the feet of a horse would be optimal.

## Related patterns (same book page)

- [[Imbue Lapis]] `— → —`
- [[Reclaim Amethyst]] `— → —`
- [[Mold Amel]] `— → —`
- [[Enchant Skin]] `entity, num → —`
- [[Enchant Fists]] `entity, num → —`
- [[Enchant Arms]] `entity, bool → —`
- [[Enhancement Purification]] `entity, num → —`

## Sources

- 📖 **Guidebook:** [[Lapisworks Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `?` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:moar_speed`
