---
aliases: ["Write Grimoire"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/storage]
kind: "Spell"
family: "Spell"
topics: ["Reading & Writing"]
mod: "Hexical"
op_id: "hexical:write_grimoire"
signature: "aqwqaeaqa"
start_direction: "WEST"
input: "pattern, list"
output: ""
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Grimoires]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Write Grimoire

✦ **Spell** · [[Hexical]] · 📖 [[Grimoires]] · Items

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 85.0,27.0 65.7,60.5 85.0,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 85.0,27.0 65.7,60.5 85.0,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="53.3" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.4,60.5 53.3,56.3 53.3,64.8" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqwqaeaqa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern, list` ([[List]], [[Pattern]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:write_grimoire`

```text
before   … │ pattern │ list  ◂ top
after    …                   ◂ top
```

## Description

Associates a pattern to a list of patterns in the **Grimoire** in my offhand.

## Related patterns (same book page)

- [[Erase Grimoire]] `pattern → —`
- [[Archivist Reflection]] `— → list of patterns`

## Sources

- 📖 **Guidebook:** [[Grimoires]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Write Grimoire](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#items/grimoire@hexical:write_grimoire)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpGrimoireWrite` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:write_grimoire`
