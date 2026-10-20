---
aliases: ["Coalesce"]
tags: [hexcasting/spell, mod/hexwright, family/spell, topic/hexwright]
kind: "Spell"
family: "Spell"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:coalescing_gambit"
signature: "dadadeewe"
start_direction: "NORTH_EAST"
input: "vec, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Coalescer]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Coalesce

✦ **Spell** · [[Hexwright]] · 📖 [[Coalescer]] · Hex Wright › Fire & Essence

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="33.2" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,42.1 36.9,51.9 29.5,47.7" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dadadeewe` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:coalescing_gambit`

```text
before   … │ vec │ num  ◂ top
after    …              ◂ top
```

## Description

Causes the Coalescer at the given position to create the number of items specified.
Costs 1 Dust

## Sources

- 📖 **Guidebook:** [[Coalescer]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `HexwrightActions$3` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:coalescing_gambit`
