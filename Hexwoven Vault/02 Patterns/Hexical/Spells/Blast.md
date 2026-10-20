---
aliases: ["Blast"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:blast"
signature: "qwqqadadadewweewwe"
start_direction: "SOUTH_EAST"
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

# Blast

✦ **Spell** · [[Hexical]] · 📖 [[Block Mimicry]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="160" height="170" viewBox="0 0 160.2 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="154.3" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="139.4" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="154.3" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="139.4" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="154.3" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/><circle cx="139.4" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="35.6,123.5 50.4,149.2 80.1,149.2 109.8,149.2 124.6,123.5 109.8,97.8 94.9,123.5 80.1,97.8 65.3,123.5 50.4,97.8 35.6,123.5 20.8,97.8 35.6,72.2 50.4,46.5 65.3,20.8 94.9,20.8 109.8,46.5 124.6,72.2 139.4,97.8 124.6,123.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:564;--dur:5.3s" points="35.6,123.5 50.4,149.2 80.1,149.2 109.8,149.2 124.6,123.5 109.8,97.8 94.9,123.5 80.1,97.8 65.3,123.5 50.4,97.8 35.6,123.5 20.8,97.8 35.6,72.2 50.4,46.5 65.3,20.8 94.9,20.8 109.8,46.5 124.6,72.2 139.4,97.8 124.6,123.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.6" y1="123.5" x2="40.3" y2="131.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.8,137.7 37.5,133.4 43.2,130.1" fill="#fc77be"/><circle cx="35.6" cy="123.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwqqadadadewweewwe` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:blast`

```text
before   … │ item entity  ◂ top
after    …                ◂ top
```

## Description

A specialized variant of [[Cook]] that simulates a **Blast Furnace** instead, making it unable to process anything but ores but at a faster rate, making the spell cheaper.

## Related patterns (same book page)

- [[Push]] `vector, vector → —`
- [[Dispense]] `item entity, vector, vector → —`
- [[Cook]] `item entity → —`
- [[Roast]] `item entity → —`
- [[Smoke]] `item entity → —`
- [[Cut Stone]] `item entity, identifier → —`

## Sources

- 📖 **Guidebook:** [[Block Mimicry]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Blast](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/block_mimicry@hexical:blast)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpCook` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:blast`
