---
aliases: ["Mirror Purification Z"]
tags: [hexcasting/pattern, mod/hexstruction, family/purification, topic/blocks, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["World: Blocks, Terrain & Weather", "Sets, Maps & Special Iotas"]
mod: "HexStruction"
op_id: "hexstruction:mirror_front_back"
signature: "aeqeawaeqeaqqwe"
start_direction: "EAST"
input: "structure"
output: "structure"
cost: ""
iotas_in: ["[[Structure]]"]
iotas_out: ["[[Structure]]"]
entry: "[[Structure Iota]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Mirror Purification Z

◇ **Pattern** · [[HexStruction]] · 📖 [[Structure Iota]] · Patterns

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,145.5 94.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 94.4,24.5 59.4,24.5 24.5,24.5 41.9,54.7 24.5,85.0 41.9,115.3 24.5,145.5 59.4,145.5 76.9,115.3 59.4,85.0 41.9,54.7 59.4,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:559;--dur:4.5s" points="59.4,145.5 94.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 94.4,24.5 59.4,24.5 24.5,24.5 41.9,54.7 24.5,85.0 41.9,115.3 24.5,145.5 59.4,145.5 76.9,115.3 59.4,85.0 41.9,54.7 59.4,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="145.5" x2="70.6" y2="145.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="78.6,145.5 70.6,149.4 70.6,141.7" fill="#fc77be"/><circle cx="59.4" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeqeawaeqeaqqwe` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `structure` ([[Structure]])
- **Leaves:** `structure` ([[Structure]])
- **ID:** `hexstruction:mirror_front_back`

```text
before   … │ structure  ◂ top
after    … │ structure  ◂ top
```

## Description

Flips a stored Structure front-to-back, or across the Z-axis.

## Related patterns (same book page)

- [[Mirror Purification X]] `structure → structure`
- [[Mirror Purification Y]] `structure → structure`
- [[Rotation Purification Clockwise Y]] `structure → structure`
- [[Rotation Purification Counter-Clockwise Y]] `structure → structure`
- [[Rotation Purification Clockwise X]] `structure → structure`
- [[Rotation Purification Counter-Clockwise X]] `structure → structure`
- [[Rotation Purification Clockwise Z]] `structure → structure`
- [[Rotation Purification Counter-Clockwise Z]] `structure → structure`
- [[Bounding Purification]] `structure → [number]`
- [[Structure Transformation Purification]] `structure → [number]`

## Sources

- 📖 **Guidebook:** [[Structure Iota]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexStruction on Modrinth](https://modrinth.com/mod/dGRSMTTM)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMirrorFrontBack` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexstruction:mirror_front_back`
