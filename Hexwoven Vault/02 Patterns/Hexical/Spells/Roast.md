---
aliases: ["Roast"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:roast"
signature: "aqqwwqqawdadedad"
start_direction: "NORTH_WEST"
input: "item entity"
output: ""
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Block Mimicry]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Roast

✦ **Spell** · [[Hexical]] · 📖 [[Block Mimicry]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="53.5,49.3 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 116.5,49.3 100.7,76.6 85.0,49.3 69.3,76.6 53.5,49.3 69.3,22.0 85.0,49.3 100.7,22.0 116.5,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:535;--dur:4.8s" points="53.5,49.3 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 116.5,49.3 100.7,76.6 85.0,49.3 69.3,76.6 53.5,49.3 69.3,22.0 85.0,49.3 100.7,22.0 116.5,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="49.3" x2="48.5" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.9,34.3 51.5,38.8 45.5,42.3" fill="#fc77be"/><circle cx="53.5" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqqwwqqawdadedad` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:roast`

```text
before   … │ item entity  ◂ top
after    …                ◂ top
```

## Description

A specialized variant of [[Cook]] that simulates a **Campfire** instead. This has a much more limited set of possibilities; I am unsure why I would ever use this.

## Related patterns (same book page)

- [[Push]] `vector, vector → —`
- [[Dispense]] `item entity, vector, vector → —`
- [[Cook]] `item entity → —`
- [[Smoke]] `item entity → —`
- [[Blast]] `item entity → —`
- [[Cut Stone]] `item entity, identifier → —`

## Sources

- 📖 **Guidebook:** [[Block Mimicry]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Roast](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/block_mimicry@hexical:roast)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpCook` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:roast`
