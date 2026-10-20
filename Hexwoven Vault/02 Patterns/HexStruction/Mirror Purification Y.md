---
aliases: ["Mirror Purification Y"]
tags: [hexcasting/pattern, mod/hexstruction, family/purification, topic/blocks, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["World: Blocks, Terrain & Weather", "Sets, Maps & Special Iotas"]
mod: "HexStruction"
op_id: "hexstruction:mirror_vertical"
signature: "eawaeqeawaeaw"
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

# Mirror Purification Y

◇ **Pattern** · [[HexStruction]] · 📖 [[Structure Iota]] · Patterns

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="69.3,103.8 100.7,103.8 116.5,131.1 132.2,103.8 148.0,76.6 116.5,76.6 100.7,49.3 69.3,49.3 53.5,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,103.8 85.0,76.6 100.7,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:441;--dur:3.9s" points="69.3,103.8 100.7,103.8 116.5,131.1 132.2,103.8 148.0,76.6 116.5,76.6 100.7,49.3 69.3,49.3 53.5,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,103.8 85.0,76.6 100.7,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="69.3" y1="103.8" x2="79.3" y2="103.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="86.6,103.8 79.3,107.3 79.3,100.4" fill="#fc77be"/><circle cx="69.3" cy="103.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eawaeqeawaeaw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `structure` ([[Structure]])
- **Leaves:** `structure` ([[Structure]])
- **ID:** `hexstruction:mirror_vertical`

```text
before   … │ structure  ◂ top
after    … │ structure  ◂ top
```

## Description

Flips a stored Structure top-to-bottom, or across the Y-axis.

## Related patterns (same book page)

- [[Mirror Purification Z]] `structure → structure`
- [[Mirror Purification X]] `structure → structure`
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
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMirrorVertical` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexstruction:mirror_vertical`
