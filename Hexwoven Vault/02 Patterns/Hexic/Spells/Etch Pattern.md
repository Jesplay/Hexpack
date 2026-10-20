---
aliases: ["Etch Pattern"]
tags: [hexcasting/spell, mod/hexic, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hexic"
op_id: "hexic:mkmacro"
signature: "deeeeewwdedqqqdedwawwdeeeee"
start_direction: "EAST"
input: "pattern, str?, entity, iota"
output: ""
cost: "cost 5 dust; replacing/removing macros is free."
iotas_in: ["[[Entity]]", "[[Pattern]]", "[[String]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Pattern Etching]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Etch Pattern

✦ **Spell** · [[Hexic]] · 📖 [[Pattern Etching]] · Patterns

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="129" viewBox="0 0 170.0 129.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.6" cy="18.6" r="2"/><circle cx="45.2" cy="18.6" r="2"/><circle cx="71.7" cy="18.6" r="2"/><circle cx="98.3" cy="18.6" r="2"/><circle cx="124.8" cy="18.6" r="2"/><circle cx="151.4" cy="18.6" r="2"/><circle cx="5.3" cy="41.6" r="2"/><circle cx="31.9" cy="41.6" r="2"/><circle cx="58.4" cy="41.6" r="2"/><circle cx="85.0" cy="41.6" r="2"/><circle cx="111.6" cy="41.6" r="2"/><circle cx="138.1" cy="41.6" r="2"/><circle cx="164.7" cy="41.6" r="2"/><circle cx="18.6" cy="64.6" r="2"/><circle cx="45.2" cy="64.6" r="2"/><circle cx="71.7" cy="64.6" r="2"/><circle cx="98.3" cy="64.6" r="2"/><circle cx="124.8" cy="64.6" r="2"/><circle cx="151.4" cy="64.6" r="2"/><circle cx="5.3" cy="87.6" r="2"/><circle cx="31.9" cy="87.6" r="2"/><circle cx="58.4" cy="87.6" r="2"/><circle cx="85.0" cy="87.6" r="2"/><circle cx="111.6" cy="87.6" r="2"/><circle cx="138.1" cy="87.6" r="2"/><circle cx="164.7" cy="87.6" r="2"/><circle cx="18.6" cy="110.6" r="2"/><circle cx="45.2" cy="110.6" r="2"/><circle cx="71.7" cy="110.6" r="2"/><circle cx="98.3" cy="110.6" r="2"/><circle cx="124.8" cy="110.6" r="2"/><circle cx="151.4" cy="110.6" r="2"/></g><polyline class="hexpat-base" points="45.2,64.6 71.7,64.6 58.4,87.6 31.9,87.6 18.6,64.6 31.9,41.6 58.4,41.6 71.7,64.6 85.0,87.6 98.3,110.6 71.7,110.6 58.4,87.6 85.0,87.6 98.3,64.6 85.0,41.6 58.4,41.6 71.7,18.6 98.3,18.6 85.0,41.6 71.7,64.6 98.3,64.6 124.8,64.6 151.4,64.6 138.1,87.6 111.6,87.6 98.3,64.6 111.6,41.6 138.1,41.6 151.4,64.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:744;--dur:6.0s" points="45.2,64.6 71.7,64.6 58.4,87.6 31.9,87.6 18.6,64.6 31.9,41.6 58.4,41.6 71.7,64.6 85.0,87.6 98.3,110.6 71.7,110.6 58.4,87.6 85.0,87.6 98.3,64.6 85.0,41.6 58.4,41.6 71.7,18.6 98.3,18.6 85.0,41.6 71.7,64.6 98.3,64.6 124.8,64.6 151.4,64.6 138.1,87.6 111.6,87.6 98.3,64.6 111.6,41.6 138.1,41.6 151.4,64.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.2" y1="64.6" x2="53.7" y2="64.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="59.8,64.6 53.7,67.5 53.7,61.7" fill="#fc77be"/><circle cx="45.2" cy="64.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeeeewwdedqqqdedwawwdeeeee` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern, str?, entity, iota` ([[Entity]], [[Pattern]], [[String]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **Cost:** cost 5 dust; replacing/removing macros is free.
- **ID:** `hexic:mkmacro`

```text
before   … │ pattern │ str? │ entity │ iota  ◂ top
after    …                                   ◂ top
```

## Description

Etches a macro into the given item entity with the given name (optional); removes a macro if the hex (topmost argument) is null. New macros cost 5 dust; replacing/removing macros is free.

## Sources

- 📖 **Guidebook:** [[Pattern Etching]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Spell):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexic:mkmacro`
