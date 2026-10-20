---
aliases: ["Pluck"]
tags: [hexcasting/spell, mod/yaha, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "YAHA"
op_id: "yaha:potion_to_item"
signature: "dqqqqqedwda"
start_direction: "EAST"
input: "entity"
output: ""
cost: "Costs one Charged Amethyst."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Yaha Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Pluck

✦ **Spell** · [[YAHA]] · 📖 [[Yaha Spells]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 46.4,94.0 65.7,127.4 104.3,127.4 123.6,94.0 104.3,60.5 65.7,60.5 46.4,27.0 85.0,27.0 123.6,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="27.0,60.5 65.7,60.5 46.4,94.0 65.7,127.4 104.3,127.4 123.6,94.0 104.3,60.5 65.7,60.5 46.4,27.0 85.0,27.0 123.6,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqqqqqedwda` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs one Charged Amethyst.
- **ID:** `yaha:potion_to_item`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

Plucks a potion out of the air, turning it into an item entity. Costs one [[Amethyst|Charged Amethyst]].

From what I have discerned, the definition of a "potion", as far as Nature is concerned, is simply "some fluid within some clear vessel that shatters." I do not yet see a use for my findings.

## Related patterns (same book page)

- [[Bury Item]] `vector, entity → —`

## Sources

- 📖 **Guidebook:** [[Yaha Spells]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPotionToItem` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `yaha:potion_to_item`
