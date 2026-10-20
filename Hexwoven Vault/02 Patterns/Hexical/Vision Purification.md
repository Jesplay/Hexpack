---
aliases: ["Vision Purification"]
tags: [hexcasting/pattern, mod/hexical, family/purification, topic/visuals]
kind: "Pattern"
family: "Purification"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:translate_dye"
signature: "wdwwaawwewdwwewwdwwe"
start_direction: "EAST"
input: "dye"
output: "vector"
cost: ""
iotas_in: ["[[Pigment]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Dyes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vision Purification

◇ **Pattern** · [[Hexical]] · 📖 [[Dyes]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="113" height="170" viewBox="0 0 113.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.2" cy="18.0" r="2"/><circle cx="30.9" cy="18.0" r="2"/><circle cx="56.7" cy="18.0" r="2"/><circle cx="82.5" cy="18.0" r="2"/><circle cx="108.2" cy="18.0" r="2"/><circle cx="18.0" cy="40.4" r="2"/><circle cx="43.8" cy="40.4" r="2"/><circle cx="69.6" cy="40.4" r="2"/><circle cx="95.4" cy="40.4" r="2"/><circle cx="5.2" cy="62.7" r="2"/><circle cx="30.9" cy="62.7" r="2"/><circle cx="56.7" cy="62.7" r="2"/><circle cx="82.5" cy="62.7" r="2"/><circle cx="108.2" cy="62.7" r="2"/><circle cx="18.0" cy="85.0" r="2"/><circle cx="43.8" cy="85.0" r="2"/><circle cx="69.6" cy="85.0" r="2"/><circle cx="95.4" cy="85.0" r="2"/><circle cx="5.2" cy="107.3" r="2"/><circle cx="30.9" cy="107.3" r="2"/><circle cx="56.7" cy="107.3" r="2"/><circle cx="82.5" cy="107.3" r="2"/><circle cx="108.2" cy="107.3" r="2"/><circle cx="18.0" cy="129.6" r="2"/><circle cx="43.8" cy="129.6" r="2"/><circle cx="69.6" cy="129.6" r="2"/><circle cx="95.4" cy="129.6" r="2"/><circle cx="5.2" cy="152.0" r="2"/><circle cx="30.9" cy="152.0" r="2"/><circle cx="56.7" cy="152.0" r="2"/><circle cx="82.5" cy="152.0" r="2"/><circle cx="108.2" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="30.9,62.7 56.7,62.7 82.5,62.7 69.6,85.0 56.7,107.3 43.8,129.6 69.6,129.6 56.7,107.3 43.8,85.0 30.9,62.7 43.8,40.4 56.7,18.0 69.6,40.4 82.5,62.7 95.4,85.0 82.5,107.3 69.6,129.6 56.7,152.0 43.8,129.6 30.9,107.3 18.0,85.0 30.9,62.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:5.9s" points="30.9,62.7 56.7,62.7 82.5,62.7 69.6,85.0 56.7,107.3 43.8,129.6 69.6,129.6 56.7,107.3 43.8,85.0 30.9,62.7 43.8,40.4 56.7,18.0 69.6,40.4 82.5,62.7 95.4,85.0 82.5,107.3 69.6,129.6 56.7,152.0 43.8,129.6 30.9,107.3 18.0,85.0 30.9,62.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.9" y1="62.7" x2="39.2" y2="62.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="45.1,62.7 39.2,65.5 39.2,59.8" fill="#fc77be"/><circle cx="30.9" cy="62.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwwaawwewdwwewwdwwe` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `dye` ([[Pigment]])
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexical:translate_dye`

```text
before   … │ dye     ◂ top
after    … │ vector  ◂ top
```

## Description

Translates a dye color into a vector, where each component is from 0 to 1 and represents the red, green, and blueness of the dye respectively.

## Related patterns (same book page)

- [[Chromatic Purification]] `id/vector/entity → dye/null`
- [[Dye]] `vector/entity, dye → —`

## Sources

- 📖 **Guidebook:** [[Dyes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Vision Purification](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/dyes@hexical:translate_dye)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTranslateDye` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:translate_dye`
