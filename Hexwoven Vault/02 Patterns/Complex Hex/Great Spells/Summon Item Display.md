---
aliases: ["Summon Item Display"]
tags: [hexcasting/great-spell, mod/complex-hex, family/great-spell, topic/visuals, hexcasting/needs-enlightenment]
kind: "Great Spell"
family: "Great Spell"
topics: ["Visuals, Sound & UI"]
mod: "Complex Hex"
op_id: "complexhex:summonitemdisplay"
signature: "wqwqwqwqwqwaqedeaaedeq"
start_direction: "SOUTH_EAST"
input: "vector, string"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[String]]"]
iotas_out: []
entry: "[[BIT Manipulation]]"
needs_enlightenment: true
per_world: true
kind_source: "tag"
topic_source: "page"
in_book: true
---

# Summon Item Display

⭐ **Great Spell** · [[Complex Hex]] · 📖 [[BIT Manipulation]] · Patterns › Great Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** 🔒 needs **enlightenment** ([[Progression - Enlightenment & Great Spells|how]]) · 🎲 strokes **differ per world**: find the scroll

> [!abstract] ⭐ Great Spell
> This Great Spell is **world-altering**, has **per-world strokes** and usually needs **enlightenment**. See [[Pattern Naming Guide]] · [[Patterns by Family#Great Spell|all Great Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 100.7,49.3 85.0,76.6 53.5,76.6 69.3,103.8 85.0,76.6 116.5,76.6 100.7,103.8 69.3,103.8 53.5,131.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:724;--dur:6.0s" points="22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 100.7,49.3 85.0,76.6 53.5,76.6 69.3,103.8 85.0,76.6 116.5,76.6 100.7,103.8 69.3,103.8 53.5,131.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="76.6" x2="27.1" y2="85.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="30.7,91.6 24.1,87.0 30.1,83.6" fill="#fc77be"/><circle cx="22.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwqwqwqwqwaqedeaaedeq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

> [!warning] Per-world pattern
> This pattern's strokes are **randomised for every world**. The shape above is only the default. Find the real one on scrolls in loot chests (see [[Progression - Enlightenment & Great Spells]]).

## Stack effect

- **Takes:** `vector, string` ([[Vector]], [[String]])
- **Leaves:** `nothing` (—)
- **ID:** `complexhex:summonitemdisplay`

```text
before   … │ vector │ string  ◂ top
after    …                    ◂ top
```

## Description

Summons an Item Display entity at the given position, displaying the given item (The string must be the internal name i.e "minecraft:stick", not "Stick").
Costs about a [[Amethyst|Charged Amethyst]]

## Related patterns (same book page)

- [[Summon Block Display]] `vector, string → —`
- [[Summon Text Display]] `vector, string → —`
- [[Entity Purification BIT]] `vector → entity`
- [[Zone Distillation BIT]] `vector, number → [entity]`
- [[Zone Distillation Not BIT]] `vector, number → [entity]`
- [[Translate BIT]] `BIT, vec → —`
- [[Rotate BIT]] `BIT, quat → —`
- [[Scale BIT]] `BIT, vec → —`
- [[Kill BIT]] `BIT → —`
- [[Get BIT 4x4]] `BIT → matrix`
- [[Set BIT 4x4]] `BIT, matrix → —`

## Sources

- 📖 **Guidebook:** [[BIT Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Summon Item Display](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/great_spells/bits@complexhex:summonitemdisplay)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Great Spell):** the mod tags it as a per-world (Great Spell) pattern
- 🔧 **Pattern ID:** `complexhex:summonitemdisplay`
