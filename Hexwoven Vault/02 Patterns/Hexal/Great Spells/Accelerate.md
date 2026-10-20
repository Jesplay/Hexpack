---
aliases: ["Accelerate"]
tags: [hexcasting/great-spell, mod/hexal, family/great-spell, topic/blocks, hexcasting/needs-enlightenment]
kind: "Great Spell"
family: "Great Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexal"
op_id: "hexal:tick"
signature: "wwwdwdwwwawqqeqwqqwqeqwqq"
start_direction: "SOUTH_EAST"
input: "vec"
output: ""
cost: "costs a tenth of an Amethyst Dust, plus a thousandth of an Amethyst Dust for each time it has previously been cast on that block in this hex."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Accelerate (Book Entry)]]"
needs_enlightenment: true
per_world: true
kind_source: "tag"
topic_source: "page"
in_book: true
---

# Accelerate

⭐ **Great Spell** · [[Hexal]] · 📖 [[Accelerate (Book Entry)]] · Patterns › Great Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** 🔒 needs **enlightenment** ([[Progression - Enlightenment & Great Spells|how]]) · 🎲 strokes **differ per world**: find the scroll

> [!abstract] ⭐ Great Spell
> This Great Spell is **world-altering**, has **per-world strokes** and usually needs **enlightenment**. See [[Pattern Naming Guide]] · [[Patterns by Family#Great Spell|all Great Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="113" height="170" viewBox="0 0 113.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.0" cy="18.0" r="2"/><circle cx="43.8" cy="18.0" r="2"/><circle cx="69.6" cy="18.0" r="2"/><circle cx="95.4" cy="18.0" r="2"/><circle cx="5.2" cy="40.4" r="2"/><circle cx="30.9" cy="40.4" r="2"/><circle cx="56.7" cy="40.4" r="2"/><circle cx="82.5" cy="40.4" r="2"/><circle cx="108.2" cy="40.4" r="2"/><circle cx="18.0" cy="62.7" r="2"/><circle cx="43.8" cy="62.7" r="2"/><circle cx="69.6" cy="62.7" r="2"/><circle cx="95.4" cy="62.7" r="2"/><circle cx="5.2" cy="85.0" r="2"/><circle cx="30.9" cy="85.0" r="2"/><circle cx="56.7" cy="85.0" r="2"/><circle cx="82.5" cy="85.0" r="2"/><circle cx="108.2" cy="85.0" r="2"/><circle cx="18.0" cy="107.3" r="2"/><circle cx="43.8" cy="107.3" r="2"/><circle cx="69.6" cy="107.3" r="2"/><circle cx="95.4" cy="107.3" r="2"/><circle cx="5.2" cy="129.6" r="2"/><circle cx="30.9" cy="129.6" r="2"/><circle cx="56.7" cy="129.6" r="2"/><circle cx="82.5" cy="129.6" r="2"/><circle cx="108.2" cy="129.6" r="2"/><circle cx="18.0" cy="152.0" r="2"/><circle cx="43.8" cy="152.0" r="2"/><circle cx="69.6" cy="152.0" r="2"/><circle cx="95.4" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="30.9,40.4 43.8,62.7 56.7,85.0 69.6,107.3 82.5,129.6 56.7,129.6 30.9,129.6 43.8,107.3 56.7,85.0 69.6,62.7 82.5,40.4 56.7,40.4 30.9,40.4 18.0,62.7 30.9,85.0 18.0,107.3 30.9,129.6 43.8,152.0 69.6,152.0 82.5,129.6 95.4,107.3 82.5,85.0 95.4,62.7 82.5,40.4 69.6,18.0 43.8,18.0 30.9,40.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:670;--dur:6.0s" points="30.9,40.4 43.8,62.7 56.7,85.0 69.6,107.3 82.5,129.6 56.7,129.6 30.9,129.6 43.8,107.3 56.7,85.0 69.6,62.7 82.5,40.4 56.7,40.4 30.9,40.4 18.0,62.7 30.9,85.0 18.0,107.3 30.9,129.6 43.8,152.0 69.6,152.0 82.5,129.6 95.4,107.3 82.5,85.0 95.4,62.7 82.5,40.4 69.6,18.0 43.8,18.0 30.9,40.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.9" y1="40.4" x2="35.1" y2="47.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="38.0,52.6 32.6,48.9 37.5,46.1" fill="#fc77be"/><circle cx="30.9" cy="40.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwwdwdwwwawqqeqwqqwqeqwqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

> [!warning] Per-world pattern
> This pattern's strokes are **randomised for every world**. The shape above is only the default. Find the real one on scrolls in loot chests (see [[Progression - Enlightenment & Great Spells]]).

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** costs a tenth of an Amethyst Dust, plus a thousandth of an Amethyst Dust for each time it has previously been cast on that block in this hex.
- **ID:** `hexal:tick`

```text
before   … │ vec  ◂ top
after    …        ◂ top
```

## Description

Causes the block at the passed vector to perform its normal actions faster in the instant that this spell is cast.

Casting it costs a tenth of an [[Amethyst|Amethyst Dust]], plus a thousandth of an [[Amethyst|Amethyst Dust]] for each time it has previously been cast on that block in this hex.

## Sources

- 📖 **Guidebook:** [[Accelerate (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Accelerate](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/great_spells/tick@hexal:tick)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Great Spell):** the mod tags it as a per-world (Great Spell) pattern
- 🔧 **Pattern ID:** `hexal:tick`
