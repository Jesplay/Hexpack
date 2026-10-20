---
aliases: ["Conjure Solid Color"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Lapisworks"
op_id: "lapisworks:conjure_color"
signature: "qqaa"
start_direction: "NORTH_EAST"
input: "vec, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Enchanted Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Solid Color

✦ **Spell** · [[Lapisworks]] · 📖 [[Enchanted Spells]] · Lapisworks

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,132.0 119.4,85.0 92.3,38.0 38.0,38.0 65.1,85.0 92.3,38.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="92.3,132.0 119.4,85.0 92.3,38.0 38.0,38.0 65.1,85.0 92.3,38.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="132.0" x2="101.0" y2="117.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.2,106.2 106.1,119.9 95.8,114.0" fill="#fc77be"/><circle cx="92.3" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqaa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:conjure_color`

```text
before   … │ vec │ num  ◂ top
after    …              ◂ top
```

## Description

Conjures a fragile but solid block of the specified color at the provided position (which must be within range) for two amethyst dust. More explanation on the next page.

The number states the index (starting from 0) of the dye I wish to use if they were all sorted alphabetically. All mappings:
0 - **Black**      1 - **Blue**
2 - **Brown**      3 - **Cyan**
4 - **Gray**       5 - **Green**
6 - **Light Blue** 7 - **Light Gray**
8 - **Lime**       9 - **Magenta**
10 - **Orange**    11 - **Pink**
12 - **Purple**    13 - **Red**
14 - White     15 - **Yellow**
Additionally, the block glows with the *dye* of my [[Pigments|pigment]]'s color and purple if no such dye exists.

## Related patterns (same book page)

- [[Dispel Conjureable]] `vec → —`
- [[Pull]] `entity, vec, num → —`

## Sources

- 📖 **Guidebook:** [[Enchanted Spells]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `ConjureColor` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:conjure_color`
