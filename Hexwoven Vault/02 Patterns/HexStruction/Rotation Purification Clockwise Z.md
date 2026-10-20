---
aliases: ["Rotation Purification Clockwise Z"]
tags: [hexcasting/pattern, mod/hexstruction, family/purification, topic/blocks, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["World: Blocks, Terrain & Weather", "Sets, Maps & Special Iotas"]
mod: "HexStruction"
op_id: "hexstruction:rotate_clockwise_z"
signature: "aeqeawaeqeaeed"
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

# Rotation Purification Clockwise Z

◇ **Pattern** · [[HexStruction]] · 📖 [[Structure Iota]] · Patterns

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="88" height="170" viewBox="0 0 87.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.0" cy="18.0" r="2"/><circle cx="43.8" cy="18.0" r="2"/><circle cx="69.6" cy="18.0" r="2"/><circle cx="5.2" cy="40.4" r="2"/><circle cx="30.9" cy="40.4" r="2"/><circle cx="56.7" cy="40.4" r="2"/><circle cx="82.5" cy="40.4" r="2"/><circle cx="18.0" cy="62.7" r="2"/><circle cx="43.8" cy="62.7" r="2"/><circle cx="69.6" cy="62.7" r="2"/><circle cx="5.2" cy="85.0" r="2"/><circle cx="30.9" cy="85.0" r="2"/><circle cx="56.7" cy="85.0" r="2"/><circle cx="82.5" cy="85.0" r="2"/><circle cx="18.0" cy="107.3" r="2"/><circle cx="43.8" cy="107.3" r="2"/><circle cx="69.6" cy="107.3" r="2"/><circle cx="5.2" cy="129.6" r="2"/><circle cx="30.9" cy="129.6" r="2"/><circle cx="56.7" cy="129.6" r="2"/><circle cx="82.5" cy="129.6" r="2"/><circle cx="18.0" cy="152.0" r="2"/><circle cx="43.8" cy="152.0" r="2"/><circle cx="69.6" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="43.8,107.3 69.6,107.3 56.7,85.0 69.6,62.7 56.7,40.4 69.6,18.0 43.8,18.0 18.0,18.0 30.9,40.4 18.0,62.7 30.9,85.0 18.0,107.3 43.8,107.3 56.7,129.6 43.8,152.0 30.9,129.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:387;--dur:4.2s" points="43.8,107.3 69.6,107.3 56.7,85.0 69.6,62.7 56.7,40.4 69.6,18.0 43.8,18.0 18.0,18.0 30.9,40.4 18.0,62.7 30.9,85.0 18.0,107.3 43.8,107.3 56.7,129.6 43.8,152.0 30.9,129.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="43.8" y1="107.3" x2="52.1" y2="107.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="58.0,107.3 52.1,110.2 52.1,104.5" fill="#fc77be"/><circle cx="43.8" cy="107.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeqeawaeqeaeed` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `structure` ([[Structure]])
- **Leaves:** `structure` ([[Structure]])
- **ID:** `hexstruction:rotate_clockwise_z`

```text
before   … │ structure  ◂ top
after    … │ structure  ◂ top
```

## Description

Rotates a stored Structure 90-degrees clockwise about the Z-axis.

## Related patterns (same book page)

- [[Mirror Purification Z]] `structure → structure`
- [[Mirror Purification X]] `structure → structure`
- [[Mirror Purification Y]] `structure → structure`
- [[Rotation Purification Clockwise Y]] `structure → structure`
- [[Rotation Purification Counter-Clockwise Y]] `structure → structure`
- [[Rotation Purification Clockwise X]] `structure → structure`
- [[Rotation Purification Counter-Clockwise X]] `structure → structure`
- [[Rotation Purification Counter-Clockwise Z]] `structure → structure`
- [[Bounding Purification]] `structure → [number]`
- [[Structure Transformation Purification]] `structure → [number]`

## Sources

- 📖 **Guidebook:** [[Structure Iota]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexStruction on Modrinth](https://modrinth.com/mod/dGRSMTTM)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpRotateClockwiseZ` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexstruction:rotate_clockwise_z`
