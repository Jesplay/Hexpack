---
aliases: ["Dispel Conjureable"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Lapisworks"
op_id: "lapisworks:erase_color"
signature: "qqqaq"
start_direction: "NORTH_EAST"
input: "vec"
output: ""
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Enchanted Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Dispel Conjureable

✦ **Spell** · [[Lapisworks]] · 📖 [[Enchanted Spells]] · Lapisworks

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="110.0,121.6 135.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 85.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="110.0,121.6 135.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 85.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="110.0" y1="121.6" x2="118.0" y2="107.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="123.8,97.8 122.8,110.5 113.2,105.0" fill="#fc77be"/><circle cx="110.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqaq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:erase_color`

```text
before   … │ vec  ◂ top
after    …        ◂ top
```

## Description

Annihilates a block in range that would be cheap to break with [[Break Block]] for negligible cost, leaving no breaking or hex particles at all.

## Related patterns (same book page)

- [[Conjure Solid Color]] `vec, num → —`
- [[Pull]] `entity, vec, num → —`

## Sources

- 📖 **Guidebook:** [[Enchanted Spells]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `EraseColor` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:erase_color`
