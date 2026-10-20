---
aliases: ["Collect Block"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:break_silk"
signature: "aqaeaqdeeweweedq"
start_direction: "EAST"
input: "vector"
output: ""
cost: "Costs about half of an Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Specialized Breaking]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Collect Block

✦ **Spell** · [[Hexical]] · 📖 [[Specialized Breaking]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="93.7,54.3 128.4,54.3 111.0,24.3 76.3,24.3 93.7,54.3 76.3,84.4 111.0,84.4 128.4,54.3 145.7,84.4 128.4,114.4 93.7,114.4 59.0,114.4 41.6,84.4 24.3,54.3 41.6,24.3 76.3,24.3 59.0,54.3 76.3,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:590;--dur:4.8s" points="93.7,54.3 128.4,54.3 111.0,24.3 76.3,24.3 93.7,54.3 76.3,84.4 111.0,84.4 128.4,54.3 145.7,84.4 128.4,114.4 93.7,114.4 59.0,114.4 41.6,84.4 24.3,54.3 41.6,24.3 76.3,24.3 59.0,54.3 76.3,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.7" y1="54.3" x2="104.8" y2="54.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.8,54.3 104.8,58.1 104.8,50.5" fill="#fc77be"/><circle cx="93.7" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaeaqdeeweweedq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about half of an Amethyst Dust.
- **ID:** `hexical:break_silk`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Breaks a block gently, perserving the block better than standard mining. Costs about half of an [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Extract Block]] `vector, number → —`

## Sources

- 📖 **Guidebook:** [[Specialized Breaking]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpBreakSilk` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:break_silk`
